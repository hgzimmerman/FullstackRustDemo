use crate::user::UserResponse;
use chrono::NaiveDateTime;
use identifiers::post::PostUuid;
use identifiers::thread::ThreadUuid;
use identifiers::user::UserUuid;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostResponse {
    pub uuid: PostUuid,
    pub author: UserResponse,
    pub created_date: NaiveDateTime,
    pub modified_date: Option<NaiveDateTime>,
    pub content: String,
    pub censored: bool,
    pub children: Vec<PostResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewPostRequest {
    pub author_uuid: UserUuid,
    pub thread_uuid: ThreadUuid,
    pub parent_uuid: Option<PostUuid>,
    pub content: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditPostRequest {
    pub uuid: PostUuid,
    pub thread_uuid: ThreadUuid,
    pub content: String,
}
