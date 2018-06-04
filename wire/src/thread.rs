use user::UserResponse;
use post::PostResponse;
use chrono::NaiveDateTime;
use identifiers::thread::ThreadUuid;
use identifiers::forum::ForumUuid;
use identifiers::user::UserUuid;


/// Used when requesting that a thread be created.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewThreadRequest {
    pub forum_id: ForumUuid,
    pub author_id: UserUuid,
    pub title: String,
    pub post_content: String,
}

/// Used when viewing an individual thread.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadResponse {
    pub id: ThreadUuid,
    pub forum_id: ForumUuid,
    pub title: String,
    pub author: UserResponse,
    pub posts: PostResponse,
    pub created_date: NaiveDateTime,
    pub locked: bool,
}

/// Used when returning a list of threads for perusing
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinimalThreadResponse {
    pub id: ThreadUuid,
    pub title: String,
    pub author: UserResponse,
    pub created_date: NaiveDateTime,
    pub locked: bool,
}
