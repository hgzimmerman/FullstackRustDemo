use wire::post::PostResponse;
use datatypes::user::UserData;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MinimalNewPostData {
    pub author_id: i32,
    pub content: String,
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct PostData {
    pub id: i32,
    pub author: UserData,
    pub created_date: i64,
    pub modified_date: Option<i64>,
    pub content: String,
    pub censored: bool,
    pub children: Vec<PostData>,
}

impl From<PostResponse> for PostData {
    fn from(response: PostResponse) -> Self{
        PostData {
            id: response.id,
            author: UserData::from(response.author),
            created_date: response.created_date,
            modified_date: response.modified_date,
            content: response.content,
            censored: response.censored,
            children: response.children.into_iter().map(PostData::from).collect()
        }
    }
}

impl PostData {
    pub fn merge_childless(&mut self, other: PostData) {
        self.content = other.content;
        self.modified_date = other.modified_date;
        self.author = other.author; // There is a very remote chance that user related data changed between updates
    }
}