use chrono::NaiveDateTime;
use chrono::Utc;
use requests_and_responses::thread::MinimalThreadResponse;
use datatypes::user::UserData;

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalThreadData {
    pub id: i32,
    pub title: String,
    pub author: UserData,
    pub created_date: NaiveDateTime,
    //    pub replies: i32,
    pub locked: bool,
}

impl Default for MinimalThreadData {
    fn default() -> MinimalThreadData {
        MinimalThreadData {
            id: 0,
            title: "".into(),
            author: UserData::default(),
            created_date: Utc::now().naive_utc(),
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
