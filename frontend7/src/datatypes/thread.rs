//use chrono::NaiveDateTime;
//use chrono::Utc;
use wire::thread::MinimalThreadResponse;
use wire::thread::NewThreadRequest;
use wire::thread::ThreadResponse;
use datatypes::user::UserData;
use datatypes::post::PostData;

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalThreadData {
    pub id: i32,
    pub title: String,
    pub author: UserData,
    pub created_date: i64,
    //    pub replies: i32,
    pub locked: bool,
}

impl Default for MinimalThreadData {
    fn default() -> MinimalThreadData {
        MinimalThreadData {
            id: 0,
            title: "".into(),
            author: UserData::default(),
            created_date: 0,
            locked: false,
        }
    }
}

impl From<MinimalThreadResponse> for MinimalThreadData {
    fn from(response: MinimalThreadResponse) -> Self {
        MinimalThreadData {
            id: response.id,
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
    pub fn attach_info(&self, forum_id: i32, user_id: i32) -> NewThreadRequest {
        NewThreadRequest {
            forum_id,
            author_id: user_id,
            title: self.title.clone(),
            post_content: self.post_content.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThreadData {
    pub id: i32,
    pub forum_id: i32,
    pub title: String,
    pub author: UserData,
    pub posts: PostData,
    pub created_date: i64,
    pub locked: bool,
}
impl From<ThreadResponse> for ThreadData {
    fn from(response: ThreadResponse) -> Self {
        ThreadData {
            id: response.id,
            forum_id: response.forum_id,
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
    pub is_selected: bool
}
impl From<MinimalThreadData> for SelectableMinimalThreadData {
    fn from(minimal_thread_data: MinimalThreadData) -> Self {
        SelectableMinimalThreadData {
            minimal_thread_data,
            is_selected: false
        }
    }
}

impl Into<MinimalThreadData> for SelectableMinimalThreadData {
    fn into(self) -> MinimalThreadData {
        self.minimal_thread_data
    }
}
