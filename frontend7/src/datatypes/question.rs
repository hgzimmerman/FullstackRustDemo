use datatypes::answer::AnswerData;
use datatypes::user::UserData;
use wire::question::QuestionResponse;

#[derive(Clone, Debug, Default)]
pub struct QuestionData {
    pub id: i32,
    pub bucket_id: i32,
    pub question_text: String,
    pub author: UserData,
    pub answers: Vec<AnswerData>,
}

impl From<QuestionResponse> for QuestionData {
    fn from(response: QuestionResponse) -> QuestionData {
        QuestionData {
            id: response.id,
            bucket_id: response.bucket_id,
            question_text: response.question_text,
            author: UserData::from(response.author),
            answers: response.answers.into_iter().map(AnswerData::from).collect(),
        }
    }
}


#[derive(Clone, Debug, Default)]
pub struct NewQuestionData {
    pub question_text: String
}
