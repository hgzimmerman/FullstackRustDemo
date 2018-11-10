use crate::user::UserResponse;
use identifiers::{
    answer::AnswerUuid,
    question::QuestionUuid
};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AnswerResponse {
    pub uuid: AnswerUuid,
    pub answer_text: Option<String>,
    pub author: Option<UserResponse>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewAnswerRequest {
    pub question_uuid: QuestionUuid,
    pub answer_text: Option<String>,
}
