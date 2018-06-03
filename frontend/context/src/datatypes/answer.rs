

use datatypes::user::UserData;
use wire::answer::AnswerResponse;
use identifiers::answer::AnswerUuid;


#[derive(Clone, Debug, Default)]
pub struct AnswerData {
    pub id: AnswerUuid,
    pub answer_text: Option<String>,
    pub author: UserData,
}

impl From<AnswerResponse> for AnswerData {
    fn from(response: AnswerResponse) -> Self {
        AnswerData {
            id: response.id,
            answer_text: response.answer_text,
            author: UserData::from(response.author),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NewAnswerData {
    pub answer_text: Option<String>,
}
