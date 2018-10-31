use crate::schema::answers;
use crate::user::User;
use crate::question::Question;
//use error::JoeResult;
use uuid::Uuid;
use diesel::pg::PgConnection;
use error::JoeResult;
use identifiers::answer::AnswerUuid;
use crate::calls::prelude::*;
use crate::schema;

#[derive(Debug, Clone, Identifiable, Queryable, Associations, TypeName)]
#[primary_key(uuid)]
#[table_name = "answers"]
//#[insertable = "NewAnswer"]
#[belongs_to(User, foreign_key = "author_uuid")]
#[belongs_to(Question, foreign_key = "question_uuid")]
pub struct Answer {
    /// Primary Key.
    pub uuid: Uuid,
    pub question_uuid: Uuid,
    pub author_uuid: Uuid,
    pub answer_text: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "answers"]
pub struct NewAnswer {
    pub author_uuid: Uuid,
    pub question_uuid: Uuid,
    pub answer_text: Option<String>,
}

#[derive(Debug)]
pub struct AnswerData {
    pub answer: Answer,
    pub user: User,
}

impl Answer {
    pub fn get_answer(uuid: AnswerUuid,conn: &PgConnection) -> JoeResult<Answer> {
        get_row::<Answer,_>(schema::answers::table, uuid.0, conn)
    }
    pub fn delete_answer(uuid: AnswerUuid, conn: &PgConnection) -> JoeResult<Answer> {
        delete_row::<Answer,_>(schema::answers::table, uuid.0, conn)
    }
    pub fn create_answer(new: NewAnswer, conn: &PgConnection) -> JoeResult<Answer> {
        create_row::<Answer, NewAnswer,_>(schema::answers::table, new, conn)
    }
}
