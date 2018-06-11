//! The auth module deals with authenticating users on the site.
//! Passwords are hashed with scrypt.
//! JSON Web Tokens are returned to the user.
//! JWTs should be included in http requests to the site under the `Authorization` header.
//! Because of signature checking, the server can trust the contents of the JWT payload and can use them to guard access to protected APIs.
//! FromRequest is implemented for some dummy user types.
//! They will only succeed in creating themselves if the JWT contains the role the user type corresponds to.
//! By specifying one of these user types on a routable method, rocket will not route the request to it unless it can resolve the role in the jwt in the request header.


#![feature(use_extern_macros)]

extern crate wire;
extern crate crypto;
extern crate rocket;
extern crate rocket_contrib;
extern crate frank_jwt;
extern crate chrono;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate simplelog;

extern crate identifiers;
extern crate error;

extern crate rand;

mod jwt;
mod password;
mod banned_set;


pub use jwt::user_authorization;
pub use jwt::{ServerJwt};

pub use password::{hash_password, verify_hash};

pub use banned_set::BannedSet;

use rand::{Rng};
use chrono::Utc;

use wire::user::Jwt;

use log::{info, warn};
use identifiers::user::UserUuid;


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

    pub fn from_user_supplied_string(key: String) -> Secret {
        if key.len() <= 128 {
            panic!("The secret key must be equal to or greater than 128 characters.")
        } else if key.len() < 256 {
            warn!("The secret key should be longer than 256 characters. It is {} characters long", key.len());
        }
        Secret(key)
    }
}





#[cfg(test)]
mod test {
    use super::*;
    use wire::user::UserRole;
//    use wire::user::UserResponse;
//    use wire::user::NewUserRequest;

//    use rocket::local::Client;
//    use wire::user::UpdateDisplayNameRequest;
//    use serde_json;
//    use rocket::http::Header;
//    use rocket::http::ContentType;
//    use init_rocket;
//    use db::Creatable;
//    use uuid::Uuid;



//    #[test]
//    fn jwt_integration_test() {
//        let pool = db::init_pool();
//
//        let user_name: String = "UserName-JwtIntegrationTest".into();
//
//        // Delete the entry to avoid
//        let conn = Conn::new(pool.get().unwrap());
//        let _ = User::delete_user_by_name(user_name.clone(), &conn);
//
//        // Create a user
//        let new_user = NewUserResponse {
//            user_name: user_name.clone(),
//            display_name: "DisplayName".into(),
//            plaintext_password: "TestPassword".into(),
//        };
//        let _: UserResponse = User::create(new_user.into(), &conn)
//            .unwrap()
//            .into();
//
//        // Log in as user
//        let login_request: LoginRequest = LoginRequest {
//            user_name: user_name.clone(),
//            password: "TestPassword".into(),
//        };
//        let rocket = init_rocket();
//        let client = Client::new(rocket).expect(
//            "valid rocket instance",
//        );
//
//        let mut response = client
//            .post("/api/auth/login/")
//            .header(ContentType::JSON)
//            .body(&serde_json::to_string(&login_request)
//                .unwrap())
//            .dispatch();
//
//        //login(login_request, secret.0, &conn);
//        assert_eq!(response.status(), Status::Ok);
//        let jwt_string: String = response
//            .body()
//            .unwrap()
//            .into_string()
//            .unwrap();
//
//
//
//        let request_body: UpdateDisplayNameRequest = UpdateDisplayNameRequest {
//            user_name: user_name.clone(),
//            new_display_name: "new name".into(),
//        };
//
//        let response = client
//            .put("/api/user/")
//            .header(ContentType::JSON)
//            .header(Header::new("Authorization", jwt_string.clone()))
//            .body(
//                serde_json::to_string(&request_body)
//                    .unwrap(),
//            )
//            .dispatch();
//        assert_eq!(response.status(), Status::Ok);
//
//
//
//        let _ = User::delete_user_by_name(user_name, &conn);
//    }


    #[test]
    fn password_hash_and_verify() {
        let plaintext: &str = "12345";
        let hash_1: String = hash_password(plaintext).unwrap();
        info!("hashed_password: {}", hash_1);
        match verify_hash(&plaintext, &hash_1) {
            Ok(_) => {}
            Err(e) => {
                panic!("error: {}", e);
            }
        }
    }

    #[test]
    fn jwt() {
        let secret = Secret("secret".to_string());

        let sub = UserUuid::default();
        let jwt = Jwt {
            sub,
            user_roles: vec![UserRole::Unprivileged],
            exp: Utc::now().naive_utc(),
            iat: Utc::now().naive_utc(),
        };
        let jwt = ServerJwt(jwt);

        let jwt_string: String = jwt.encode_jwt_string(&secret).unwrap();
        let _jwt: ServerJwt = match ServerJwt::decode_jwt_string(&jwt_string, &secret) {
            Ok(j) => j,
            Err(e) => {
                panic!("{:?}",e);
            }
        };
    }
    #[test]
    fn jwt_tampering_detected() {
        let secret = Secret("secret".to_string());
        // create a normal jwt
        let sub = UserUuid::default();
        let jwt = Jwt {
            sub,
            user_roles: vec![UserRole::Unprivileged],
            exp: Utc::now().naive_utc(),
            iat: Utc::now().naive_utc(),
        };
        let jwt = ServerJwt(jwt);

        let jwt_string: String = jwt.encode_jwt_string(&secret).unwrap();
        // alter the username of a copy of the accepted jwt
        let mut altered_jwt: ServerJwt = jwt.clone();
        altered_jwt.0.user_roles = vec![UserRole::Admin];
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
        if let Ok(_) = ServerJwt::decode_jwt_string(&synthesized_jwt_string, &secret) {
            panic!("Should not be able to decode this modified jwt.");
        }
    }

}
