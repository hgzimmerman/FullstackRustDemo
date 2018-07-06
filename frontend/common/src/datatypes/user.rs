
use wire::user::*;
use identifiers::user::UserUuid;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct UserData {
    pub user_name: String,
    pub display_name: String,
    pub uuid: UserUuid,
}

impl From<UserResponse> for UserData {
    fn from(response: UserResponse) -> Self {
        UserData {
            user_name: response.user_name,
            display_name: response.display_name,
            uuid: response.uuid,
        }
    }
}
