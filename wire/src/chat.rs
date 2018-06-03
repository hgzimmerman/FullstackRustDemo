use user::UserResponse;
use identifiers::chat::ChatUuid;


/// Messages will be sent in a separate response.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChatResponse {
    pub id: ChatUuid,
    pub name: String,
    pub leader: UserResponse,
    pub members: Vec<UserResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MinimalChatResponse {
    pub id: ChatUuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewChatRequest {
    pub leader_id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChatUserAssociationRequest {
    pub chat_id: ChatUuid,
    pub user_id: i32,
}
