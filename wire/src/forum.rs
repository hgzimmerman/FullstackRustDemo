use identifiers::forum::ForumUuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ForumResponse {
    pub id: ForumUuid,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewForumRequest {
    pub title: String,
    pub description: String,
}
