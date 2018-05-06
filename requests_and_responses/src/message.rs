use user::UserResponse;
//use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageResponse {
    pub id: i32,
    pub author: UserResponse,
    pub reply: Option<Box<MessageResponse>>,
    pub content: String,
    pub date: u64,
}



#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewMessageRequest {
    pub author_id: i32,
    pub chat_id: i32,
    pub reply_id: Option<i32>,
    pub content: String,
}
