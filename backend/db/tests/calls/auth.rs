use calls::user::{UserFixture, self};
use common::setup::*;
use diesel::PgConnection;
//use db::user::{User, NewUser};
use db::auth;
use wire::login::LoginRequest;
use auth_lib::ServerJwt;

#[test]
fn fail_login_invalid_password() {
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let bad_login = LoginRequest {
            user_name: fixture.admin_user.user_name.clone(),
            password: "Invalid Password".to_string(),
        };
        auth::login(bad_login, &fixture.secret, conn)
            .expect_err("Should have failed to log the user in");
    })
}

#[test]
fn fail_login_invalid_username() {
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let bad_login = LoginRequest {
            user_name: "Non-existent username".to_string(),
            password: "Inconsequential password".to_string(),
        };
        auth::login(bad_login, &fixture.secret, conn)
            .expect_err("Should have failed to log the user in");
    })
}

#[test]
fn successful_login() {
    use wire::user::UserRole;
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let login_request = LoginRequest {
            user_name: fixture.admin_user.user_name.clone(),
            password: user::PASSWORD.to_string()
        };
        let jwt_string: String = auth::login(login_request, &fixture.secret, conn)
            .expect("Should have logged the user in");

        let jwt =  ServerJwt::decode_jwt_string(jwt_string.as_str(), &fixture.secret )
            .expect("Decoded jwt token");
        assert_eq!(jwt.sub.0, fixture.admin_user.uuid);
        let expected_roles: Vec<UserRole> = fixture.admin_user.roles.clone().into_iter().map(UserRole::from).collect();
        assert_eq!(jwt.user_roles, expected_roles);
    })
}

#[test]
fn successful_reauth() {
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let login_request = LoginRequest {
            user_name: fixture.admin_user.user_name.clone(),
            password: user::PASSWORD.to_string()
        };
        let jwt_string: String = auth::login(login_request, &fixture.secret, conn)
            .expect("Should have logged the user in");

        let jwt =  ServerJwt::decode_jwt_string(jwt_string.as_str(), &fixture.secret )
            .expect("Decoded jwt token");
        let jwt: ServerJwt = ServerJwt(jwt);

        auth::reauth(jwt, &fixture.secret).expect("New JWT should be provided");
    })
}
