use crate::{
    user::UserResponse,
    answer::AnswerResponse
};
use identifiers::{
    question::QuestionUuid,
    bucket::BucketUuid
};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuestionResponse {
    pub uuid: QuestionUuid,
    pub bucket_uuid: BucketUuid,
    pub question_text: String,
    pub author: Option<UserResponse>,
    pub answers: Vec<AnswerResponse>,
    pub on_floor: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewQuestionRequest {
    pub bucket_uuid: BucketUuid,
    pub question_text: String,
}
