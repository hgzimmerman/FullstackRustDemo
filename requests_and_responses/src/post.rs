use user::UserResponse;
//use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostResponse {
    pub id: i32,
    pub author: UserResponse,
    pub created_date: u64,
    pub modified_date: Option<u64>,
    pub content: String,
    pub censored: bool,
    pub children: Vec<PostResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewPostRequest {
    pub author_id: i32,
    pub thread_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditPostRequest {
    pub id: i32,
    pub thread_id: i32,
    pub content: String,
}
