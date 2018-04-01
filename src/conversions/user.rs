use db::user::*;
use requests_and_responses::user::*;

use auth::hash_password;


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
