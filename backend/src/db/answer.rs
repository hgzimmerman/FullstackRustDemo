use schema::answers;
use db::user::User;
use db::question::Question;
//use error::JoeResult;
use uuid::Uuid;



#[derive(Debug, Clone, Identifiable, Queryable, Associations, CrdUuid, ErrorHandler)]
#[table_name = "answers"]
#[insertable = "NewAnswer"]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Question, foreign_key = "question_id")]
pub struct Answer {
    /// Primary Key.
    pub id: Uuid,
    pub question_id: Uuid,
    pub author_id: i32,
    pub answer_text: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "answers"]
pub struct NewAnswer {
    pub author_id: i32,
    pub question_id: Uuid,
    pub answer_text: Option<String>,
}

pub struct AnswerData {
    pub answer: Answer,
    pub user: User,
}
