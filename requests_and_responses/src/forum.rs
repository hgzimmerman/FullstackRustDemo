#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ForumResponse {
    pub title: String,
    pub description: String,
    pub id: i32
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewForumRequest {
    pub title: String,
    pub description: String
}

