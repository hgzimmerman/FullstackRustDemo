#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    question: String,
    answer: Option<String>,
    author: String,
    answered_by: Option<String>,
    id: usize
}