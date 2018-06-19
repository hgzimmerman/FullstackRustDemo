use common::setup::*;
use diesel::PgConnection;
use db::user::{User, NewUser};
use test::Bencher;




use db::CreatableUuid;
use db::RetrievableUuid;
use identifiers::user::UserUuid;

use testing_fixtures::fixtures::user::UserFixture;
use testing_fixtures::fixtures::user::ADMIN_USER_NAME;
use testing_fixtures::EmptyFixture;

/// Just tests the fixture
#[test]
fn user_fixture_test() {
    setup(|fixture: &UserFixture, _conn: &PgConnection| {
        assert_eq!(fixture.admin_user.user_name.as_str(), ADMIN_USER_NAME)
    })
}


#[test]
fn delete_user() {
    use db::RetrievableUuid;
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        User::delete_user_by_name(fixture.normal_user.user_name.clone(), conn)
            .expect("Should have deleted user");
        User::get_by_uuid(fixture.normal_user.uuid, conn)
            .expect_err("User should have been deleted");
    })
}


#[test]
fn add_role() {
    use wire::user::UserRole;

    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let user_uuid = UserUuid(fixture.normal_user.uuid);
        let user_role = UserRole::Publisher;

        assert!(!fixture.normal_user.roles.contains(&user_role.into()));
        User::add_role_to_user(user_uuid, user_role, conn)
            .expect("add role of publisher to user");
        let changed_user: User = User::get_by_uuid(fixture.normal_user.uuid, conn)
            .expect("User should be retrieved");

        assert!(changed_user.roles.contains(&user_role.into()));

    })
}

#[test]
fn ban_status() {
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let user_uuid = UserUuid(fixture.normal_user.uuid);

        User::set_ban_status(user_uuid, true, conn)
            .expect("user should be banned");
        let changed_user: User = User::get_by_uuid(fixture.normal_user.uuid, conn)
            .expect("User should be retrieved");

        assert!(changed_user.banned);

        let is_user_banned: bool = User::is_user_banned(user_uuid, conn).unwrap();
        assert!(is_user_banned);

        User::set_ban_status(user_uuid, false, conn)
            .expect("user should be unbanned");
        let changed_user: User = User::get_by_uuid(fixture.normal_user.uuid, conn)
            .expect("User should be retrieved");

        let is_user_banned: bool = User::is_user_banned(user_uuid, conn).unwrap();
        assert!(!is_user_banned);
        assert!(!changed_user.banned);
    })
}


#[test]
fn get_by_user_name() {
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let user: User = User::get_user_by_user_name(&fixture.normal_user.user_name, conn)
            .expect("get user by user name");
        assert_eq!(user, fixture.normal_user);
    })
}


#[test]
fn get_by_user_role() {
    use wire::user::UserRole;

    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let user_role = UserRole::Admin;

        let users: Vec<User> = User::get_users_with_role(user_role, conn)
            .expect("expected to get users with a given role");
        assert!(users.contains(&fixture.admin_user));
        assert!(!users.contains(&fixture.normal_user));

        let user_role = UserRole::Unprivileged;
        let users: Vec<User> = User::get_users_with_role(user_role, conn)
            .expect("expected to get users with a given role");
        assert!(users.contains(&fixture.admin_user));
        assert!(users.contains(&fixture.normal_user));
    })
}

#[test]
fn get_users() {
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        let users: Vec<User> = User::get_users(3, conn)
            .expect("get users");
        assert!(users.len() <= 3);
        assert!(users.contains(&fixture.admin_user));
        assert!(users.contains(&fixture.normal_user));

        let users: Vec<User> = User::get_users(0, conn)
            .expect("get users");
        assert_eq!(users.len(), 0);
    })
}



#[bench]
fn get_user_bench(b: &mut Bencher) {
    use db::RetrievableUuid;
    setup(|fixture: &UserFixture, conn: &PgConnection| {
        b.iter(
            || User::get_by_uuid(fixture.admin_user.uuid, &conn),
        );
    });
}

#[bench]
fn crd_user_bench(b: &mut Bencher) {
    use db::RetrievableUuid;
    setup(|_fixture: &EmptyFixture, conn: &PgConnection| {

        fn crud(conn: &PgConnection) {
            const USER_NAME: &'static str = "OldDisplayName";
            const NEW_DISPLAY_NAME: &'static str = "NewDisplayName";

            // Delete the entry to avoid
            let _ = User::delete_user_by_name(USER_NAME.into(), &conn);

            // Create a user
            let new_admin_user = NewUser {
                user_name: USER_NAME.to_string(),
                display_name: String::from("Admin"),
                password_hash: String::from("Invalid Password Hash"),
                failed_login_count: 0,
                banned: false,
                roles: vec![1,2,3,4] // Has all privileges
            };

            let response: User = User::create(new_admin_user, &conn)
                .unwrap();
            assert_eq!(USER_NAME, response.user_name.as_str());

            // Get User
            let response: User = User::get_by_uuid(response.uuid, &conn)
                .unwrap();
            assert_eq!(USER_NAME, response.user_name.as_str());


            let response: User = User::update_user_display_name(USER_NAME.to_string(), NEW_DISPLAY_NAME.to_string(), conn)
                .unwrap();
            assert_eq!(NEW_DISPLAY_NAME, response.display_name.as_str());


            // Delete the entry
            let _ = User::delete_user_by_name(USER_NAME.to_string(), &conn);
        }
        b.iter(
            || crud(&conn),
        );
    });
}
