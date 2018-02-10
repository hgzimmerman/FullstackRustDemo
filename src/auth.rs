use rocket::request::Form;
use rocket::response::content::Html;
use rocket::http::Cookies;
use rocket::Route;
use std::collections::HashMap;

use crypto::scrypt;
use super::Routable;

use frank_jwt::{Algorithm, encode, decode};
use frank_jwt;
use chrono::{NaiveDateTime, DateTime, Utc, Duration};
use rocket_contrib::Json;
use user::User;
use db::Conn;
use rocket::response::Responder;
use rand::{self, Rng};
use rocket::State;
use rocket::http::Status;
use rocket::http::ContentType;
use rocket::Response;
use serde_json;
use routes::user::UserRole;
use std::io;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

#[derive(Debug)]
pub enum LoginError {
    UsernameDoesNotExist,
    IncorrectPassword,
    PasswordHashingError(&'static str),
    JwtError(JwtError),
    UpdateUserFailed,
    OtherError(&'static str)
}

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

pub fn hash_password(password: &str) ->  io::Result<String> {
    let params: scrypt::ScryptParams = scrypt::ScryptParams::new(10, 10, 10);
    scrypt::scrypt_simple(password, &params)
}

pub fn verify_hash(plaintext: &str, expected_hash: &str) -> Result<bool, &'static str> {
    scrypt::scrypt_check(plaintext, expected_hash)
}

pub enum RoleError {
    InsufficientRights
}

pub struct NormalUser{
    user_name: String
}
impl NormalUser {
    pub fn from_jwt(jwt: &Jwt) -> Result<NormalUser, RoleError> {
        if jwt.user_roles.contains(&UserRole::Unprivileged){
            Ok(NormalUser{
                user_name: jwt.user_name.clone()
            })
        }
        else {
            Err(RoleError::InsufficientRights)
        }
    }
}
pub struct AdminUser {
    user_name: String
}
impl AdminUser {
    pub fn from_jwt(jwt: &Jwt) -> Result<AdminUser, RoleError> {
        if jwt.user_roles.contains(&UserRole::Admin){
            Ok(AdminUser{
                user_name: jwt.user_name.clone()
            })
        }
        else {
            Err(RoleError::InsufficientRights)
        }
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        };
        // You can get the state secret from another request guard
        let secret: String = match request.guard::<State<Secret>>() {
            Outcome::Success(s) => s.0.clone(),
            _ => return Outcome::Failure((Status::BadRequest, ()))
        };

        let key = keys[0];
        let jwt: Jwt = match Jwt::decode_jwt_string(key.to_string(), &secret) {
            Ok(j) => j,
            Err(_) => return Outcome::Failure((Status::BadRequest, ()))
        };

        match AdminUser::from_jwt(&jwt) {
            Ok(admin) => Outcome::Success(admin),
            Err(e) => Outcome::Forward(())
        }
    }
}

pub struct ModeratorUser {
    user_name: String
}
impl ModeratorUser {
    pub fn from_jwt(jwt: &Jwt) -> Result<ModeratorUser, RoleError> {
        if jwt.user_roles.contains(&UserRole::Moderator){
            Ok(ModeratorUser{
                user_name: jwt.user_name.clone()
            })
        }
        else {
            Err(RoleError::InsufficientRights)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub user_name: String,
    pub token_key: String,// The token key may not be needed
    pub user_roles: Vec<UserRole>,
    pub token_expire_date: NaiveDateTime
}

impl Jwt {
    pub fn encode_jwt_string(&self, secret: &String) -> Result<String, JwtError> {
        let header = json!({});
        use rocket_contrib::Value;

        let payload: Value = match serde_json::to_value(self) {
            Ok(x) => x,
            Err(e) => return Err(JwtError::SerializeError)
        };
        match encode(header, secret, &payload, Algorithm::HS256) {
            Ok(x) => return Ok(x),
            Err(e) => return Err(JwtError::EncodeError)
        }
    }

    pub fn decode_jwt_string(jwt_str: String, secret: &String) -> Result<Jwt, JwtError> {
        let (header, payload) = match decode(&jwt_str, secret, Algorithm::HS256) {
            Ok(x) => x,
            Err(e) => return Err(JwtError::DecodeError)
        };
        let jwt: Jwt = match serde_json::from_value(payload) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::DeserializeError)
        };
        Ok(jwt)
    }

    
}



#[derive(Debug, Clone)]
pub enum JwtError {
    DecodeError,
    EncodeError,
    DeserializeError,
    SerializeError,
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