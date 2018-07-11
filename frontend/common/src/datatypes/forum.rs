use wire::forum::ForumResponse;
use identifiers::forum::ForumUuid;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ForumData {
    pub uuid: ForumUuid,
    pub title: String,
    pub description: String,
}

impl From<ForumResponse> for ForumData {
    fn from(response: ForumResponse) -> Self {
        ForumData {
            uuid: response.uuid,
            title: response.title,
            description: response.description,
        }
    }
}

