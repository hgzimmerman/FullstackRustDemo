//! The auth module deals with authenticating users on the site.
//! Passwords are hashed with scrypt.
//! JSON Web Tokens are returned to the user.
//! JWTs should be included in http requests to the site under the `Authorization` header.
//! Because of signature checking, the server can trust the contents of the JWT payload and can use them to guard access to protected APIs.
//! FromRequest is implemented for some dummy user types.
//! They will only succeed in creating themselves if the JWT contains the role the user type corresponds to.
//! By specifying one of these user types on a routable method, rocket will not route the request to it unless it can resolve the role in the jwt in the request header.


mod jwt;
mod password;
mod banned_set;

pub use self::jwt::user_authorization;
pub use self::jwt::{ServerJwt, JwtError};

pub use self::password::{hash_password, verify_hash};

pub use self::banned_set::BannedSet;

use rand::{self, Rng};
use chrono::{NaiveDateTime, Utc, Duration};
use rocket::http::Status;
use rocket::Response;
use rocket::request::Request;
use rocket::response::Responder;
use db::user::User;
use db::Conn;

use wire::login::*;
use wire::user::Jwt;


/// The secret contains a random string that is generated at startup.
/// This will be different every time the server restarts.
/// This secret randomization has the effect of invalidating JWTs whenever the server is restarted.
/// The Secret is used for creating and validating JWTs.
#[derive(Debug, Clone)]
pub struct Secret(pub String);

impl Secret {
    pub fn generate() -> Secret {
        let key = rand::thread_rng()
            .gen_ascii_chars()
            .take(256)
            .collect::<String>();
        Secret(key)
    }
}



pub type LoginResult = Result<String, LoginError>;

/// Logs the user in by validating their password and returning a jwt.
pub fn login(login_request: LoginRequest, secret: String, conn: &Conn) -> LoginResult {
    info!("Logging in for user: {}", &login_request.user_name);
    // get user
    let user: User = User::get_user_by_user_name(&login_request.user_name, &conn)
        .map_err(|_| LoginError::UsernameDoesNotExist)?;

    // Check if the user is locked.
    // This will clean up any locked status if the lock has already expired.
    if user.check_if_locked(conn).map_err(
        |_| {
            LoginError::OtherError("DB error")
        },
    )?
    {
        return Err(LoginError::AccountLocked);
    }

    info!("verifing password: {}", &login_request.password);
    info!("against: {}", &user.password_hash);
    match verify_hash(&login_request.password, &user.password_hash) {
        Ok(b) => {
            if !b {
                info!("Wrong password entered for user: {}", &login_request.user_name);
                User::record_failed_login(user.id, user.failed_login_count, &conn)
                    .map_err(|_| LoginError::OtherError("Login failed, but could not set the login delay"))?;
                return Err(LoginError::IncorrectPassword);
            } else {
                info!("Password match verified");
                if user.failed_login_count > 0 {
                    info!("Resetting login count");
                    User::reset_login_failure_count(user.id, &conn)
                        .map_err(|_| LoginError::OtherError("DB error"))?;
                }
            }
        }
        Err(e) => return Err(LoginError::PasswordHashingError(e)),
    }


    // generate token
    info!("Generating JWT Expiry Date");
    let duration: Duration = Duration::days(1);
    let new_expire_date: NaiveDateTime = match Utc::now().checked_add_signed(duration) {
        Some(ndt) => ndt.naive_utc(),
        None => return Err(LoginError::OtherError("Could not calculate offset for token expiry")),
    };

    let exp_timestamp: i64 = new_expire_date.timestamp();

    info!("Creating JWT");
    let jwt = Jwt {
        //        user_name: user.user_name.clone(),
        sub: user.id.clone(),
        user_roles: user.roles
            .iter()
            .map(|role_id| (*role_id).into())
            .collect(),
        exp: exp_timestamp,
    };
    let jwt = ServerJwt(jwt);
    let jwt_string: String = match jwt.encode_jwt_string(&secret) {
        Ok(s) => s,
        Err(e) => return Err(LoginError::JwtError(e)),
    };

    Ok(jwt_string)

}


#[derive(Debug)]
pub enum LoginError {
    UsernameDoesNotExist,
    IncorrectPassword,
    AccountLocked,
    PasswordHashingError(&'static str),
    JwtError(JwtError),
    OtherError(&'static str),
}

impl<'a> Responder<'a> for LoginError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        // TODO: use the string in a custom Status for internal server error
        info!("User login failed with error: {:?}", &self);
        match self {
            LoginError::IncorrectPassword => Err(Status::Unauthorized),
            LoginError::AccountLocked => Err(Status::Unauthorized),
            LoginError::UsernameDoesNotExist => Err(Status::NotFound),
            LoginError::JwtError(_) => Err(Status::InternalServerError),
            LoginError::PasswordHashingError(_) => Err(Status::InternalServerError),
            LoginError::OtherError(_) => Err(Status::InternalServerError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use db::user::{User, UserRole};
    use requests_and_responses::user::UserResponse;
    use requests_and_responses::user::NewUserRequest;
    use db;

    use rocket::local::Client;
    use requests_and_responses::user::UpdateDisplayNameRequest;
    use serde_json;
    use rocket::http::Header;
    use rocket::http::ContentType;
    use init_rocket;
    use db::Creatable;
    #[test]
    fn login_test() {

        let pool = db::init_pool();

        // Delete the entry to avoid
        let conn = Conn::new(pool.get().unwrap());
        let _ = User::delete_user_by_name("UserName".into(), &conn);

        // Create a user
        let new_user = NewUserRequest {
            user_name: "UserName".into(),
            display_name: "DisplayName".into(),
            plaintext_password: "TestPassword".into(),
        };
        let _: UserResponse = User::create(new_user.into(), &conn)
            .unwrap()
            .into();

        // Log in as user
        let login_request: LoginRequest = LoginRequest {
            user_name: "UserName".into(),
            password: "TestPassword".into(),
        };

        let secret: Secret = Secret::generate();

        let response = login(login_request, secret.0, &conn);
        assert!(response.is_ok());

        let _ = User::delete_user_by_name("UserName".into(), &conn);
    }

    #[test]
    fn jwt_integration_test() {

        let pool = db::init_pool();

        let user_name: String = "UserName-JwtIntegrationTest".into();

        // Delete the entry to avoid
        let conn = Conn::new(pool.get().unwrap());
        let _ = User::delete_user_by_name(user_name.clone(), &conn);

        // Create a user
        let new_user = NewUserRequest {
            user_name: user_name.clone(),
            display_name: "DisplayName".into(),
            plaintext_password: "TestPassword".into(),
        };
        let _: UserResponse = User::create(new_user.into(), &conn)
            .unwrap()
            .into();

        // Log in as user
        let login_request: LoginRequest = LoginRequest {
            user_name: user_name.clone(),
            password: "TestPassword".into(),
        };
        let rocket = init_rocket();
        let client = Client::new(rocket).expect(
            "valid rocket instance",
        );

        let mut response = client
            .post("/api/auth/login/")
            .header(ContentType::JSON)
            .body(&serde_json::to_string(&login_request)
                .unwrap())
            .dispatch();

        //login(login_request, secret.0, &conn);
        assert_eq!(response.status(), Status::Ok);
        let jwt_string: String = response
            .body()
            .unwrap()
            .into_string()
            .unwrap();



        let request_body: UpdateDisplayNameRequest = UpdateDisplayNameRequest {
            user_name: user_name.clone(),
            new_display_name: "new name".into(),
        };

        let response = client
            .put("/api/user/")
            .header(ContentType::JSON)
            .header(Header::new("Authorization", jwt_string.clone()))
            .body(
                serde_json::to_string(&request_body)
                    .unwrap(),
            )
            .dispatch();
        assert_eq!(response.status(), Status::Ok);



        let _ = User::delete_user_by_name(user_name, &conn);
    }


    #[test]
    fn password_hash_and_verify() {
        use test_setup;
        test_setup();
        let plaintext = "12345";
        let hash_1 = hash_password(plaintext).unwrap();
        info!("hashed_password: {}", hash_1);
        match verify_hash(&plaintext, &hash_1) {
            Ok(_) => {}
            Err(e) => {
                info!("error: {}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn jwt() {
        use test_setup;
        test_setup();
        let secret = "secret".to_string();

        let jwt = Jwt {
            user_name: "name".to_string(),
            user_id: 1,
            user_roles: vec![UserRole::Unprivileged],
            token_expire_date: Utc::now().naive_utc(),
        };

        let jwt_string: String = jwt.encode_jwt_string(&secret).unwrap();
        let jwt: Jwt = match Jwt::decode_jwt_string(jwt_string, &secret) {
            Ok(j) => j,
            Err(e) => {
                info!("{:?}", e);
                panic!();
            }
        };
        info!("{:?}", jwt);
    }
    #[test]
    fn jwt_tampering_detected() {
        use test_setup;
        test_setup();
        let secret = "secret".to_string();
        // create a normal jwt
        let jwt = Jwt {
            user_name: "name".to_string(),
            user_id: 1,
            user_roles: vec![UserRole::Unprivileged],
            token_expire_date: Utc::now().naive_utc(),
        };
        let jwt_string: String = jwt.encode_jwt_string(&secret).unwrap();
        // alter the username of a copy of the accepted jwt
        let mut altered_jwt = jwt.clone();
        altered_jwt.user_name = "other_name".to_string();
        let altered_jwt_string = altered_jwt
            .encode_jwt_string(&secret)
            .unwrap();
        // split the JWTs
        let split_jwt: Vec<&str> = jwt_string.split(".").collect();
        let split_altered_jwt: Vec<&str> = altered_jwt_string.split(".").collect();
        // Mix together the header from the first jwt, the modified payload, and the signature.
        let normal_header: &str = split_jwt.get(0).unwrap();
        let modified_payload: &str = split_altered_jwt.get(1).unwrap();
        let normal_sig: &str = split_jwt.get(2).unwrap();
        let synthesized_jwt_string: String = format!("{}.{}.{}", normal_header, modified_payload, normal_sig);
        // The decode should fail because the signature does not correspond to the payload
        Jwt::decode_jwt_string(synthesized_jwt_string, &secret)
            .expect_err("Should not be able to decode this modified jwt.");
    }

}
