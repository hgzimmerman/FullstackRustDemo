use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewArticleRequest {
    pub title: String,
    pub body: String,
    pub author_id: i32
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub id: i32
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ArticleResponse {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub body: String,
    pub publish_date: Option<NaiveDateTime>
}