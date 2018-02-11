#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewArticleRequest {
    pub title: String,
    pub body: String,
    pub author_id: i32
}

pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub id: i32
}