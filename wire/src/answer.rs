use user::UserResponse;
use identifiers::answer::AnswerUuid;
use identifiers::question::QuestionUuid;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AnswerResponse {
    pub id: AnswerUuid,
    pub answer_text: Option<String>,
    pub author: UserResponse,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewAnswerRequest {
    pub question_id: QuestionUuid,
    pub answer_text: Option<String>,
}
