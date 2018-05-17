

use datatypes::user::UserData;


#[derive(Clone, Debug, Default)]
pub struct AnswerData {
    pub id: i32,
    pub answer_text: Option<String>,
    pub author: UserData,
}

#[derive(Clone, Debug, Default)]
pub struct NewAnswerData {
    pub answer_text: Option<String>,
}
