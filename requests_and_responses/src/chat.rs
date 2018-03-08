use user::UserResponse;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChatResponse {
    pub id: i32,
    pub name: String,
    pub leader: UserResponse,
    pub members: Vec<UserResponse>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewChatRequest {
    pub leader_id: i32,
    pub name: String
}