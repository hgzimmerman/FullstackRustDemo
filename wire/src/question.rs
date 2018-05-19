use user::UserResponse;
use answer::AnswerResponse;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QuestionResponse {
    pub id: i32,
    pub bucket_id: i32,
    pub question_text: String,
    pub author: UserResponse,
    pub answers: Vec<AnswerResponse>,
    pub on_floor: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewQuestionRequest {
    pub bucket_id: i32,
    pub question_text: String,
}
