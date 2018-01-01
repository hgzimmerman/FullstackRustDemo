#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub question: String,
    pub answer: Option<String>,
    pub author: String, // TODO, consider removing, it shouldn't need to be recorded
    pub answered_by: Option<String>,
    pub id: usize
}