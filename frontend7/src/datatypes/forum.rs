use requests_and_responses::forum::ForumResponse;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ForumData {
    pub title: String,
    pub description: String,
    pub id: i32,
}

impl From<ForumResponse> for ForumData {
    fn from(response: ForumResponse) -> Self {
        ForumData {
            title: response.title,
            description: response.description,
            id: response.id,
        }
    }
}
