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
use rocket::Request;
use rocket::Response;

#[derive(Debug)]
pub enum LoginError {
    UsernameDoesNotExist,
    IncorrectPassword,
    PasswordHashingError(&'static str),
    UpdateUserFailed,
    OtherError(&'static str)
}

static format_string: &'static str = "%Y-%m-%d %H:%M:%S";
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

use std::io;
// TODO: move this method to some auth module
pub fn hash_password(password: &str) ->  io::Result<String> {
    
    let params: scrypt::ScryptParams = scrypt::ScryptParams::new(10, 10, 10);
    scrypt::scrypt_simple(password, &params)
}

pub fn verify_hash(plaintext: &str, expected_hash: &str) -> Result<bool, &'static str> {
    scrypt::scrypt_check(plaintext, expected_hash)
}


fn generate_jwt(user_name: &str, token_key: &str, token_expire_date: NaiveDateTime, secret: &String) -> String  {
    let header = json!({});

    let expired_date: String = token_expire_date.format(format_string).to_string();
    let payload = json!({
        "user_name": user_name,
        "token_key": token_key,
        "token_expire_date": expired_date
    });
    use std::ops::Deref;
    encode(header, secret, &payload, Algorithm::HS256).unwrap()
}


// #[get("/admin", rank = 2)]
// fn login() -> Html<&'static str>{
//     Html(
//         "<form action=\"/api/login/admin\" method=\"POST\">
//         <input type=\"hidden\" name=\"username\" />
//         <input type=\"hidden\" name=\"password\" />
//         <input type=\"submit\" value=\"Login\" />
//     </form>"
//     )
// }

// #[post("/admin", data = "<form>")]
// fn login_post(form: Form<LoginStatus<DummyAuthenticator>>, cookies: Cookies) -> LoginRedirect{
//     // creates a response with either a cookie set (in case of a succesfull login)
//     // or not (in case of a failure). In both cases a "Location" header is send.
//     // the first parameter indicates the redirect URL when successful login,
//     // the second a URL for a failed login
//     form.into_inner().redirect("/api/login/admin", "/api/login/admin", cookies)
// }
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
    let jwt: String = generate_jwt(&user.user_name, &new_key, new_expire_date, &secret);
    info!("JWT created");

    // update entry with new values
    // and return the token
    return match User::update_user_jwt(user.user_name, new_key, new_expire_date, &conn) {
        Ok(_) => {
            Ok(jwt)
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
    fn generate_jwt() {
        super::generate_jwt("name", "aoeuaoeu", Utc::now().naive_utc(), &"secret".to_string());
    }

}