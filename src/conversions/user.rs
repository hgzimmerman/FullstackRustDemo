use db::user::*;
use requests_and_responses::user::*;

use auth::hash_password;

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> i32 {
        match role {
            UserRole::Unprivileged => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::Publisher => 4,
        }
    }
}

impl From<i32> for UserRole {
    fn from(number: i32) -> UserRole {
        match number {
            1 => UserRole::Unprivileged,
            2 => UserRole::Moderator,
            3 => UserRole::Admin,
            4 => UserRole::Publisher,
            _ => {
                warn!("Tried to convert an unsupported number into a user role");
                UserRole::Unprivileged
            }
        }
    }
}


impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            user_name: user.user_name,
            display_name: user.display_name,
            id: user.id,
        }
    }
}


impl From<NewUserRequest> for NewUser {
    fn from(new_user_request: NewUserRequest) -> NewUser {
        NewUser {
            user_name: new_user_request.user_name,
            display_name: new_user_request.display_name,
            password_hash: hash_password(&new_user_request.plaintext_password)
                .expect("Couldn't hash password"),
            failed_login_count: 0,
            banned: false,
            roles: vec![1],
        }
    }
}

impl From<User> for FullUserResponse {
    fn from(user: User) -> FullUserResponse {
        FullUserResponse {
            user_name: user.user_name,
            display_name: user.display_name,
            id: user.id,
            banned: user.banned,
            locked: user.locked.is_some(),
        }
    }
}
