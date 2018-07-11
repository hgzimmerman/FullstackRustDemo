//use chrono::NaiveDateTime;
//use chrono::Utc;
use wire::thread::MinimalThreadResponse;
use wire::thread::NewThreadRequest;
use wire::thread::ThreadResponse;
use datatypes::user::UserData;
use datatypes::post::PostData;
use chrono::NaiveDateTime;
use identifiers::thread::ThreadUuid;
use identifiers::forum::ForumUuid;
use identifiers::user::UserUuid;



#[derive(Clone, Debug, PartialEq)]
pub struct MinimalThreadData {
    pub uuid: ThreadUuid,
    pub title: String,
    pub author: UserData,
    pub created_date: NaiveDateTime,
    //    pub replies: i32,
    pub locked: bool,
}

impl Default for MinimalThreadData {
    fn default() -> MinimalThreadData {
        MinimalThreadData {
            uuid: ThreadUuid::default(),
            title: String::default(),
            author: UserData::default(),
            created_date: NaiveDateTime::from_timestamp(0,0),
            locked: bool::default(),
        }
    }
}

impl From<MinimalThreadResponse> for MinimalThreadData {
    fn from(response: MinimalThreadResponse) -> Self {
        MinimalThreadData {
            uuid: response.uuid,
            title: response.title,
            author: response.author.into(),
            created_date: response.created_date,
            locked: response.locked,
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct NewThreadData {
    pub title: String,
    pub post_content: String,
}

impl NewThreadData {
    pub fn attach_info(&self, forum_uuid: ForumUuid, user_uuid: UserUuid) -> NewThreadRequest {
        NewThreadRequest {
            forum_uuid,
            author_uuid: user_uuid,
            title: self.title.clone(),
            post_content: self.post_content.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThreadData {
    pub uuid: ThreadUuid,
    pub forum_uuid: ForumUuid,
    pub title: String,
    pub author: UserData,
    pub posts: PostData,
    pub created_date: NaiveDateTime,
    pub locked: bool,
}

impl Default for ThreadData {
    fn default() -> Self {
        ThreadData {
            uuid: ThreadUuid::default(),
            forum_uuid: ForumUuid::default(),
            title: String::default(),
            author: UserData::default(),
            posts: PostData::default(),
            created_date: NaiveDateTime::from_timestamp(0,0),
            locked: bool::default()
        }
    }
}

impl From<ThreadResponse> for ThreadData {
    fn from(response: ThreadResponse) -> Self {
        ThreadData {
            uuid: response.uuid,
            forum_uuid: response.forum_uuid,
            title: response.title,
            author: UserData::from(response.author),
            posts: PostData::from(response.posts),
            created_date: response.created_date,
            locked: response.locked
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectableMinimalThreadData {
    pub minimal_thread_data: MinimalThreadData,
    pub forum_uuid: ForumUuid,
    pub is_selected: bool
}
//impl From<MinimalThreadData> for SelectableMinimalThreadData {
//    fn from(minimal_thread_data: MinimalThreadData) -> Self {
//        SelectableMinimalThreadData {
//            minimal_thread_data,
//            is_selected: false
//        }
//    }
//}

impl SelectableMinimalThreadData {
    pub fn new(minimal_thread_data: MinimalThreadData, forum_uuid: ForumUuid, thread_uuid: Option<ThreadUuid>) -> Self {
        let mut is_selected: bool = false;

        if let Some(thread_uuid) = thread_uuid {
            is_selected = minimal_thread_data.uuid == thread_uuid;
        }
        SelectableMinimalThreadData {
            minimal_thread_data,
            forum_uuid,
            is_selected
        }
    }
}

impl Into<MinimalThreadData> for SelectableMinimalThreadData {
    fn into(self) -> MinimalThreadData {
        self.minimal_thread_data
    }
}
