use auth::Secret;
use diesel::PgConnection;

use db::user::{
    NewUser,
    User
};
use Fixture;

use auth::hash_password;

pub const PASSWORD: &'static str = "password";

/// This constant is present in the common crate for compile-time related reasons.
/// Because this crate rarely must be recompiled, keeping this long running function that produces
/// a static value separate from typical test creation.
lazy_static! {
    pub static ref PASSWORD_HASH: String = hash_password(PASSWORD).expect("Couldn't hash password.");
}

pub struct UserFixture {
    pub admin_user: User,
    pub normal_user: User,
    pub secret: Secret
}


pub const ADMIN_USER_NAME: &'static str = "Admin";
pub const ADMIN_DISPLAY_NAME: &'static str = "Admin";

pub const NORMAL_USER_NAME: &'static str = "Normal User";
pub const NORMAL_DISPLAY_NAME: &'static str = "Normal User";


impl Fixture for UserFixture {
    fn generate(conn: &PgConnection) -> Self {

        let secret: Secret = Secret::generate();

        let new_admin_user = NewUser {
            user_name: String::from(ADMIN_USER_NAME),
            display_name: String::from(ADMIN_DISPLAY_NAME),
            password_hash: PASSWORD_HASH.to_string(),
            failed_login_count: 0,
            banned: false,
            roles: vec![1,2,3,4] // Has all privileges
        };
        let admin_user: User = User::create_user(new_admin_user, conn).expect("Couldn't create new admin user");

        let new_normal_user = NewUser {
            user_name: String::from(NORMAL_USER_NAME),
            display_name: String::from(NORMAL_DISPLAY_NAME),
            password_hash: PASSWORD_HASH.to_string(),
            failed_login_count: 0,
            banned: false,
            roles: vec![1] // Has only basic privileges
        };
        let normal_user: User = User::create_user(new_normal_user, conn).expect("Couldn't create new normal user");


        UserFixture {
            admin_user,
            normal_user,
            secret
        }
    }
}