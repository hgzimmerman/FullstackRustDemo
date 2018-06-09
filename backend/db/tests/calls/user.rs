use common::setup::*;
use diesel::PgConnection;
use db::CreatableUuid;
use db::user::{User, NewUser};
use test::Bencher;



pub struct UserFixture {
    admin_user: User,
    normal_user: User
}


const ADMIN_USER_NAME: &'static str = "Admin";
const ADMIN_DISPLAY_NAME: &'static str = "Admin";

impl Fixture for UserFixture {
    fn generate(conn: &PgConnection) -> Self {

        let new_admin_user = NewUser {
            user_name: String::from(ADMIN_USER_NAME),
            display_name: String::from(ADMIN_DISPLAY_NAME),
            password_hash: String::from("Invalid Password Hash"),
            failed_login_count: 0,
            banned: false,
            roles: vec![1,2,3,4] // Has all privileges
        };
        let admin_user: User = User::create(new_admin_user, conn).expect("Couldn't create new admin user");

        let new_normal_user = NewUser {
            user_name: String::from("Normal"),
            display_name: String::from("Normal"),
            password_hash: String::from("Invalid Password Hash"),
            failed_login_count: 0,
            banned: false,
            roles: vec![1] // Has only basic privileges
        };
        let normal_user: User = User::create(new_normal_user, conn).expect("Couldn't create new normal user");

        UserFixture {
            admin_user,
            normal_user
        }
    }
}


#[test]
fn user_fixture_test() {
    setup(|fixture: &UserFixture, _conn: &PgConnection| {
        assert_eq!(fixture.admin_user.user_name.as_str(), ADMIN_USER_NAME)
    })
}

#[test]
fn another_user_fixture_test() {
    setup(|fixture: &UserFixture, _conn: &PgConnection| {
        assert_eq!(fixture.admin_user.user_name.as_str(), ADMIN_USER_NAME)
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
            let user_name: String = "CrudBenchTest-UserName".into();

            // Delete the entry to avoid
            let _ = User::delete_user_by_name(user_name.clone(), &conn);

            // Create a user
            let new_admin_user = NewUser {
                user_name: user_name.clone(),
                display_name: String::from("Admin"),
                password_hash: String::from("Invalid Password Hash"),
                failed_login_count: 0,
                banned: false,
                roles: vec![1,2,3,4] // Has all privileges
            };

            let response: User = User::create(new_admin_user, &conn)
                .unwrap();
            assert_eq!(user_name.clone(), response.user_name);

            // Get User
            let response: User = User::get_by_uuid(response.uuid, &conn)
                .unwrap();
            assert_eq!(user_name.clone(), response.user_name);

            let new_display_name = String::from("NewDisplayName");

            let response: User = User::update_user_display_name(user_name.clone(), new_display_name, conn)
                .unwrap();
            assert_eq!("NewDisplayName".to_string(), response.display_name);


            // Delete the entry
            let _ = User::delete_user_by_name(user_name, &conn);
        }
        b.iter(
            || crud(&conn),
        );
    });
}
