//use chrono::NaiveDateTime;
//use chrono::Utc;
use requests_and_responses::thread::MinimalThreadResponse;
use requests_and_responses::thread::NewThreadRequest;
use datatypes::user::UserData;

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

pub struct PartialNewThreadData {
    pub title: String,
    pub post_content: String,
    pub author_id: i32,
}
impl PartialNewThreadData {
    pub fn attach_forum_id(self, forum_id: i32) -> NewThreadRequest {
        NewThreadRequest {
            forum_id,
            author_id: self.author_id,
            title: self.title,
            post_content: self.post_content,
        }
    }
}
