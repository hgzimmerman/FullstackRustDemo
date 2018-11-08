use crate::user::UserResponse;
use identifiers::{
    chat::ChatUuid,
    user::UserUuid
};


/// Messages will be sent in a separate response.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChatResponse {
    pub uuid: ChatUuid,
    pub name: String,
    pub leader: UserResponse,
    pub members: Vec<UserResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MinimalChatResponse {
    pub uuid: ChatUuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewChatRequest {
    pub leader_uuid: UserUuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChatUserAssociationRequest {
    pub chat_uuid: ChatUuid,
    pub user_uuid: UserUuid,
}
