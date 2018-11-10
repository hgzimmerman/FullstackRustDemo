extern crate auth;
extern crate db;
extern crate rocket as rocket;
extern crate serde_json;
extern crate server;
extern crate testing_common;
extern crate testing_fixtures;
extern crate wire;

use rocket::{
    http::{
        ContentType,
        Header,
        Status,
    },
    local::Client,
};
use testing_common::setup::setup_client;
use testing_fixtures::fixtures::user::PASSWORD;
use wire::{
    login::LoginRequest,
    user::UpdateDisplayNameRequest,
};

use testing_fixtures::fixtures::user::UserFixture;

#[test]
fn login_auth() {
    setup_client(|fixture: &UserFixture, client: Client| {
        // Log in as user
        let login_request: LoginRequest = LoginRequest {
            user_name: fixture.normal_user.user_name.clone(),
            password: PASSWORD.to_string(),
        };

        let mut response = client
            .post("/api/auth/login/")
            .header(ContentType::JSON)
            .body(&serde_json::to_string(&login_request).unwrap())
            .dispatch();

        // TODO, Make the rocket init point to the test db
        eprintln!("{:?}", response);
        assert_eq!(response.status(), Status::Ok);
        let jwt_string: String = response.body().unwrap().into_string().unwrap();

        let request_body: UpdateDisplayNameRequest = UpdateDisplayNameRequest {
            user_name: fixture.normal_user.user_name.clone(),
            new_display_name: "new name".into(),
        };

        let response = client
            .put("/api/user/")
            .header(ContentType::JSON)
            .header(Header::new("Authorization", format!("Bearer {}", jwt_string.clone())))
            .body(serde_json::to_string(&request_body).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    });
}
