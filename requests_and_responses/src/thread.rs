use user::UserResponse;
use post::PostResponse;
use chrono::NaiveDateTime;
use post::NewPostRequest;


/// Used when requesting that a thread be created.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewThreadRequest {
    pub forum_id: i32,
    pub author_id: i32,
    pub title: String,
    pub post: NewPostRequest
}

/// Used when viewing an individual thread.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadResponse {
    pub id: i32,
    pub title: String,
    pub author: UserResponse,
    pub posts: PostResponse,
    pub created_date: NaiveDateTime,
    pub locked: bool,
}

/// Used when returning a list of threads for perusing 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinimalThreadResponse {
    pub id: i32,
    pub title: String,
    pub author: UserResponse,
    pub created_date: NaiveDateTime,
    pub locked: bool,
}