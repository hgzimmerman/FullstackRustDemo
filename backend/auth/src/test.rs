use super::*;
use wire::user::UserRole;
use chrono::Utc;

use wire::user::Jwt;

//use log::{info, warn};
use identifiers::user::UserUuid;


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
    let jwt: Jwt = Jwt {
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

