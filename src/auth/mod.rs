mod jwt;
mod password;


pub use self::jwt::user_authorization;
pub use self::jwt::{Jwt, JwtError};

// pub use self::login::login;
// pub use self::login::{LoginRequest, LoginResult, LoginError};

pub use self::password::{hash_password, verify_hash};

use rand::{self, Rng};
use chrono::{NaiveDateTime, Utc, Duration};
use rocket::http::Status;
use rocket::Response;
use rocket::request::{self, Request, FromRequest};
use rocket::response::Responder;
use user::User;
use db::Conn;


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


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String
}

pub type LoginResult = Result<String, LoginError>;

pub fn login(login_request: LoginRequest, secret: String, conn: &Conn) -> LoginResult {
    info!("Logging in for user: {}", &login_request.user_name);
    // get user
    let user: User = match User::get_user_by_user_name(&login_request.user_name, &conn){
        Some(user) => user,
        None => return Err(LoginError::UsernameDoesNotExist)
    };


    info!("verifing password: {}", &login_request.password);
    info!("against: {}", &user.password_hash);
    match verify_hash(&login_request.password, &user.password_hash) {
        Ok(b) => {
            if !b {
                info!("Wrong password entered for user: {}", &login_request.user_name);
                return Err(LoginError::IncorrectPassword);
            } else {
                info!("Password match verified");
            }
        }
        Err(e) => {
            return Err(LoginError::PasswordHashingError(e))
        }
    }
    

    // generate token
    info!("Generating JWT Expiry Date");
    let duration: Duration = Duration::days(1);
    let new_expire_date: NaiveDateTime = match Utc::now().checked_add_signed(duration) {
        Some(ndt) => ndt.naive_utc(),
        None => return Err(LoginError::OtherError("Could not calculate offset for token expiry"))
    };
    info!("Generating JWT key");
    let new_key: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(16)
        .collect::<String>();
    
    info!("Creating JWT");
    let jwt = Jwt {
        user_name: user.user_name.clone(),
        user_roles: user.roles.iter().map(|role_id| (*role_id).into()).collect(),
        token_key: new_key.clone(),
        token_expire_date: new_expire_date
    };
    let jwt_string: String = match jwt.encode_jwt_string(&secret) {
        Ok(s) => s,
        Err(e) => return Err(LoginError::JwtError(e))
    };

    // update entry with new values
    // and return the token
    info!("updating user");
    return match User::update_user_jwt(user.user_name, new_key, new_expire_date, &conn) {
        Ok(_) => {
            Ok(jwt_string)
        }
        Err(_) => Err(LoginError::UpdateUserFailed)
    }

}


#[derive(Debug)]
pub enum LoginError {
    UsernameDoesNotExist,
    IncorrectPassword,
    PasswordHashingError(&'static str),
    JwtError(JwtError),
    UpdateUserFailed,
    OtherError(&'static str)
}

impl <'a> Responder<'a> for LoginError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        // TODO: use the string in a custom Status for internal server error
        info!("User login failed with error: {:?}", &self);
        match self {
            LoginError::IncorrectPassword => Err(Status::Unauthorized),
            LoginError::UsernameDoesNotExist => Err(Status::NotFound),
            LoginError::UpdateUserFailed => Err(Status::InternalServerError),
            LoginError::JwtError(_) => Err(Status::InternalServerError),
            LoginError::PasswordHashingError(_) => Err(Status::InternalServerError),
            LoginError::OtherError(_) => Err(Status::InternalServerError)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use user::User;
    use user::UserResponse;
    use user::NewUserRequest;
    use user::delete_user_by_name;
    use user::create_user;
    use db;

    #[test]
    fn integration_test() {

        let pool = db::init_pool();

        // Delete the entry to avoid 
        let conn = Conn::new(pool.get().unwrap());
        delete_user_by_name("UserName".into(), conn);

        // Create a user
        let conn = Conn::new(pool.get().unwrap());
        let new_user = NewUserRequest {
            user_name: "UserName".into(),
            display_name: "DisplayName".into(),
            plaintext_password: "TestPassword".into() 
        };
        let response: UserResponse =  create_user(Json(new_user), conn).into_inner();
        // assert_eq!("UserName".to_string(), response.user_name);


        // Log in as user
        let conn = Conn::new(pool.get().unwrap());
        let login_request: LoginRequest = LoginRequest {
            user_name: "UserName".into(),
            password: "TestPassword".into()
        };

        // let secret: State<Secret> = State {
        //     0: &Secret::generate()
        // };
        let secret: Secret = Secret::generate();

        let response = login(login_request, secret.0, &conn);
        assert!(response.is_ok());

        let conn = Conn::new(pool.get().unwrap());
        delete_user_by_name("UserName".into(), conn);


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
                info!("error: {}",e);
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
            token_key: "aoeuaoeu".to_string(),
            user_roles: vec!(UserRole::Unprivileged),
            token_expire_date: Utc::now().naive_utc()
        };

        let jwt_string: String = super::encode_jwt_string(jwt, &secret);
        let jwt: Jwt = match super::decode_jwt_string(jwt_string, secret) {
            Ok(j) => j,
            Err(e) => {
                info!("{:?}", e);
                panic!();
            }
        };
        info!("{:?}", jwt);
    }

}


