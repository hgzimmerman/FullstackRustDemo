
use db::user::{User, NewUser};
use auth::hash_password;

/// User to be sent over the wire
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub user_name: String,
    pub display_name: String,
    pub id: i32,
}

impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            user_name: user.user_name,
            display_name: user.display_name,
            id: user.id
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserRequest {
    pub user_name: String,
    pub display_name: String,
    pub plaintext_password: String
}

impl From<NewUserRequest> for NewUser {
    fn from(new_user_request: NewUserRequest) -> NewUser {
        NewUser {
            user_name: new_user_request.user_name,
            display_name: new_user_request.display_name,
            password_hash: hash_password(&new_user_request.plaintext_password).expect("Couldn't hash password"),
            token_key: None,
            token_expire_date: None,
            roles: vec![1]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDisplayNameRequest {
    pub id: i32,
    pub new_display_name: String
}

