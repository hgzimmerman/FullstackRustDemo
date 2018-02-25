use schema::questions;
use error::WeekendAtJoesError;
use db::Conn;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use db::handle_diesel_error;
use db::user::User;
use db::bucket::Bucket;
use diesel::BelongingToDsl;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[table_name="questions"]
#[belongs_to(Bucket, foreign_key = "bucket_id")]
#[belongs_to(User, foreign_key = "author_id")]
pub struct Question {
    /// Primary Key.
    pub id: i32,
    pub bucket_id: i32,
    pub author_id: i32,
    pub question_text: String,
}

#[derive(Insertable, Debug)]
#[table_name="questions"]
pub struct NewQuestion {
    bucket_id: i32,
    author_id: i32,
    question_text: String
}

impl Question {
    /// Creates a new bucket
    pub fn create_question(new_question: NewQuestion, conn: &Conn) -> Result<Question, WeekendAtJoesError> {
        use schema::questions;

        diesel::insert_into(questions::table)
            .values(&new_question)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Question"))
    }

    /// Gets a list of all buckets.
    pub fn get_questions(conn: &Conn) -> Result<Vec<Question>, WeekendAtJoesError> {
        use schema::questions::dsl::*;
        questions 
            .load::<Question>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Question")) 
    }

    pub fn get_questions_for_bucket(owning_bucket_id: i32, conn: &Conn) -> Result<Vec<Question>, WeekendAtJoesError> {

        let bucket = Bucket::get_bucket(owning_bucket_id, &conn)?;
        Question::belonging_to(&bucket)
            .load::<Question>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Question")) 
    }

    /// Gets a bucket by id.
    pub fn get_question(question_id: i32, conn: &Conn) -> Result<Question, WeekendAtJoesError> {
        use schema::questions::dsl::*;

        // Gets the first bucket that matches the id.
        questions
            .find(question_id)
            .first::<Question>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Question"))

    }
}