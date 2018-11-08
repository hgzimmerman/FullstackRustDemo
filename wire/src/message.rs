use crate::user::UserResponse;
use chrono::NaiveDateTime;
use uuid::Uuid;
use identifiers::{
    message::MessageUuid,
    chat::ChatUuid,
    user::UserUuid
};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageResponse {
    pub uuid: MessageUuid,
    pub author: UserResponse,
    pub reply: Option<Box<MessageResponse>>,
    pub content: String,
    pub date: NaiveDateTime,
}



#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewMessageRequest {
    pub author_uuid: UserUuid,
    pub chat_uuid: ChatUuid,
    pub reply_uuid: Option<Uuid>,
    pub content: String,
}
