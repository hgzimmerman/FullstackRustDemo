use schema::answers;
use db::user::User;
use db::question::Question;
use db::Conn;
use error::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel;
use diesel::result::Error;
use std::ops::Deref;



#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[table_name = "answers"]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Question, foreign_key = "question_id")]
pub struct Answer {
    /// Primary Key.
    pub id: i32,
    pub question_id: i32,
    pub author_id: i32,
    pub answer_text: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "answers"]
pub struct NewAnswer {
    pub author_id: i32,
    pub question_id: i32,
    pub answer_text: Option<String>,
}


impl Answer {
    /// Creates a new answer
    pub fn create_answer(new_answer: NewAnswer, conn: &Conn) -> Result<Answer, WeekendAtJoesError> {
        use schema::answers;

        diesel::insert_into(answers::table)
            .values(&new_answer)
            .get_result(conn.deref())
            .map_err(Answer::handle_error)
    }

    pub fn get_answer(answer_id: i32, conn: &Conn) -> Result<Answer, WeekendAtJoesError> {
        use schema::answers::dsl::*;

        // Gets the first answer that matches the id.
        answers
            .find(answer_id)
            .first::<Answer>(conn.deref())
            .map_err(Answer::handle_error)
    }
}

impl ErrorFormatter for Answer {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError {
        handle_diesel_error(diesel_error, "Answer")
    }
}
