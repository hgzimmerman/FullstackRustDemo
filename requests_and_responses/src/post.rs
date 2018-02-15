use user::UserResponse;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
    pub id: i32,
    pub author: UserResponse,
    pub created_date: NaiveDateTime,
    pub modified_date: Option<NaiveDateTime>,
    pub content: String,
    pub censored: bool,
    pub children: Vec<PostResponse>
}