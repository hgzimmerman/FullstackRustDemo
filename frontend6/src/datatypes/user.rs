
use requests_and_responses::user::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct UserData {
    pub user_name: String,
    pub display_name: String,
    pub id: i32,
}

impl From<UserResponse> for UserData {
    fn from(response: UserResponse) -> Self {
        UserData {
            user_name: response.user_name,
            display_name: response.display_name,
            id: response.id
        }
    }
}