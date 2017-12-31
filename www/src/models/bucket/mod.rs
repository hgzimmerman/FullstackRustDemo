mod question;
pub use self::question::Question;

#[derive(Debug, Clone)]
pub struct BucketModel {
    pub active_question: Option<Question>,
    pub new_question_input: String,
    pub answer_input: String,
    pub session_id: String,
}

impl BucketModel {
    pub fn temp() -> BucketModel {

        let q = Question {
            question: "If you could change your fate, would ye?".to_string(),
            answer: None,
            author: "Me".to_string(),
            answered_by: None,
            id: 0,
        };
        BucketModel {
            active_question: Some(q),
            new_question_input: "".to_string(),
            answer_input: "".to_string(),
            session_id: String::from("bucket"),
        }
    }
}