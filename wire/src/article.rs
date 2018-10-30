use chrono::NaiveDateTime;
use crate::user::UserResponse;
use identifiers::article::ArticleUuid;
use identifiers::user::UserUuid;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewArticleRequest {
    pub title: String,
    pub body: String,
    pub author_id: UserUuid,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateArticleRequest {
    pub uuid: ArticleUuid,
    pub title: Option<String>,
    pub body: Option<String>,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MinimalArticleResponse {
    pub uuid: ArticleUuid,
    pub author_uuid: UserUuid,
    pub title: String,
    pub body: String,
    pub publish_date: Option<NaiveDateTime>,
}

/// Doesn't have the body attached.
/// This makes it ideal for returning many preview articles.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ArticlePreviewResponse {
    pub uuid: ArticleUuid,
    pub author: UserResponse,
    pub title: String,
    pub publish_date: Option<NaiveDateTime>,
}

/// All relevant information is attached.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FullArticleResponse {
    pub id: ArticleUuid,
    pub author: UserResponse,
    pub title: String,
    pub body: String,
    pub publish_date: Option<NaiveDateTime>,
}
