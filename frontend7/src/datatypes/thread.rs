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
    pub author_id: i32,
    pub forum_id: i32
}
impl Into<NewThreadRequest> for NewThreadData {
   fn into(self) -> NewThreadRequest {
       NewThreadRequest {
            forum_id: self.forum_id,
            author_id: self.author_id,
            title: self.title,
            post_content: self.post_content,
        }
   }
}
impl NewThreadData {
    pub fn attach_forum_id(self, forum_id: i32) -> NewThreadRequest {
        NewThreadRequest {
            forum_id,
            author_id: self.author_id,
            title: self.title,
            post_content: self.post_content,
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
