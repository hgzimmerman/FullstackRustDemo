use datatypes::answer::AnswerData;
use datatypes::user::UserData;

#[derive(Clone, Debug, Default)]
pub struct QuestionData {
    pub id: i32,
    pub bucket_id: i32,
    pub question_text: String,
    pub author: UserData,
    pub answers: Vec<AnswerData>,
}

#[derive(Clone, Debug, Default)]
pub struct NewQuestionData {
    pub question_text: String
}
