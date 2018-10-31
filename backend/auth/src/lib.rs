//! The auth module deals with authenticating users on the site.
//! Passwords are hashed with scrypt.
//! JSON Web Tokens are returned to the user.
//! JWTs should be included in http requests to the site under the `Authorization` header.
//! Because of signature checking, the server can trust the contents of the JWT payload and can use them to guard access to protected APIs.
//! FromRequest is implemented for some dummy user types.
//! They will only succeed in creating themselves if the JWT contains the role the user type corresponds to.
//! By specifying one of these user types on a routable method, rocket will not route the request to it unless it can resolve the role in the JWT in the request header.


//#![feature(use_extern_macros)]

// TODO Remove hard dependency on rocket, make it optional.

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
mod secret;


pub use jwt::user_authorization;
pub use jwt::{ServerJwt};

pub use password::{hash_password, verify_hash};

pub use banned_set::BannedSet;
pub use secret::Secret;

use chrono::Utc;

use wire::user::Jwt;

use log::{info, warn};
use identifiers::user::UserUuid;





#[cfg(test)]
mod test {
    use super::*;
    use wire::user::UserRole;



    #[test]
    fn password_hash_and_verify() {
        let plaintext: &str = "12345";
        let hash_1: String = hash_password(plaintext).unwrap();
        verify_hash(&plaintext, &hash_1).expect("The hash should be verified");
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
        let decoded_jwt: ServerJwt = ServerJwt::decode_jwt_string(&jwt_string, &secret).expect("JWT should be decoded from the provided string");
        assert_eq!(jwt, decoded_jwt);
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
