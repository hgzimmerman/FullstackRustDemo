use crate::user::UserResponse;
use crate::post::PostResponse;
use chrono::NaiveDateTime;
use identifiers::thread::ThreadUuid;
use identifiers::forum::ForumUuid;
use identifiers::user::UserUuid;


/// Used when requesting that a thread be created.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewThreadRequest {
    pub forum_uuid: ForumUuid,
    pub author_uuid: UserUuid, // TODO, this should be removed, JWT provides this anyway
    pub title: String,
    pub post_content: String,
}

/// Used when viewing an individual thread.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadResponse {
    pub uuid: ThreadUuid,
    pub forum_uuid: ForumUuid,
    pub title: String,
    pub author: UserResponse,
    pub posts: PostResponse,
    pub created_date: NaiveDateTime,
    pub locked: bool,
}

/// Used when returning a list of threads for perusing
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinimalThreadResponse {
    pub uuid: ThreadUuid,
    pub title: String,
    pub author: UserResponse,
    pub created_date: NaiveDateTime,
    pub locked: bool,
}
