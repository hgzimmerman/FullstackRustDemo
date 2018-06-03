use user::UserResponse;
use chrono::NaiveDateTime;
use identifiers::post::PostUuid;
use identifiers::thread::ThreadUuid;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostResponse {
    pub id: PostUuid,
    pub author: UserResponse,
    pub created_date: NaiveDateTime,
    pub modified_date: Option<NaiveDateTime>,
    pub content: String,
    pub censored: bool,
    pub children: Vec<PostResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewPostRequest {
    pub author_id: i32,
    pub thread_id: ThreadUuid,
    pub parent_id: Option<PostUuid>,
    pub content: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditPostRequest {
    pub id: PostUuid,
    pub thread_id: ThreadUuid,
    pub content: String,
}
