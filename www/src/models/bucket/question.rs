#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub question: String,
    pub answer: Option<String>,
    pub author: String,
    pub answered_by: Option<String>,
    pub id: usize
}