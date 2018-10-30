use crate::user::UserResponse;
use crate::answer::AnswerResponse;
use identifiers::question::QuestionUuid;
use identifiers::bucket::BucketUuid;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuestionResponse {
    pub uuid: QuestionUuid,
    pub bucket_uuid: BucketUuid,
    pub question_text: String,
    pub author: UserResponse,
    pub answers: Vec<AnswerResponse>,
    pub on_floor: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewQuestionRequest {
    pub bucket_uuid: BucketUuid,
    pub question_text: String,
}
