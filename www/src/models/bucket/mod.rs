mod question;
pub use self::question::Question;

pub struct BucketModel {
    user_name: String,
    active_question: Option<Question>,
    new_question_input: String,
    answer_input: String,
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