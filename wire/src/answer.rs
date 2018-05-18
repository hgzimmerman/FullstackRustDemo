use user::UserResponse;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AnswerResponse {
    pub id: i32,
    pub answer_text: Option<String>,
    pub author: UserResponse,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewAnswerRequest {
    //    pub author_id: i32,
    pub question_id: i32,
    pub answer_text: Option<String>,
}
