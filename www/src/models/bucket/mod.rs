mod question;
pub use self::question::Question;

#[derive(Debug, Clone)]
pub struct BucketModel {
    pub user_name: String,
    pub active_question: Option<Question>,
    pub new_question_input: String,
    pub answer_input: String,
}

impl BucketModel {
    pub fn temp() -> BucketModel {
        BucketModel {
            user_name: "Joe".to_string(),
            active_question: None,
            new_question_input: "".to_string(),
            answer_input: "".to_string(),
        }
    }
}