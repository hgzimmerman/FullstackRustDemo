//use chrono::NaiveDateTime;
use user::UserResponse;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewArticleRequest {
    pub title: String,
    pub body: String,
    pub author_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub id: i32,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MinimalArticleResponse {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub body: String,
    pub publish_date: Option<u64>,
}

/// Doesn't have the body attached.
/// This makes it ideal for returning many preview articles.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ArticlePreviewResponse {
    pub id: i32,
    pub author: UserResponse,
    pub title: String,
    pub publish_date: Option<u64>,
}

/// All relevant information is attached.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FullArticleResponse {
    pub id: i32,
    pub author: UserResponse,
    pub title: String,
    pub body: String,
    pub publish_date: Option<u64>,
}
