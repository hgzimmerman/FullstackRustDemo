use schema::questions;
use error::*;
use db::Conn;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use db::user::User;
use db::bucket::Bucket;
use diesel::BelongingToDsl;
use diesel::result::Error;
use db::answer::Answer;
use diesel::GroupedBy;
use rand::{thread_rng, seq};

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
    pub bucket_id: i32,
    pub author_id: i32,
    pub question_text: String
}

impl Question {
    /// Creates a new bucket
    pub fn create_question(new_question: NewQuestion, conn: &Conn) -> Result<Question, WeekendAtJoesError> {
        use schema::questions;

        diesel::insert_into(questions::table)
            .values(&new_question)
            .get_result(conn.deref())
            .map_err(Question::handle_error)
    }

    /// Gets a list of all questions across all buckets.
    pub fn get_questions(conn: &Conn) -> Result<Vec<Question>, WeekendAtJoesError> {
        use schema::questions::dsl::*;
        questions 
            .load::<Question>(conn.deref())
            .map_err(Question::handle_error)
    }

    /// Gets a random question that may have already been answered
    pub fn get_random_question(bucket_id: i32, conn: &Conn) -> Result<(Question, User, Vec<Answer>), WeekendAtJoesError> {
        use schema::users::dsl::*;

        let bucket = Bucket::get_bucket(bucket_id, &conn)?;

        no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

        let question: Question = Question::belonging_to(&bucket)
            .order(RANDOM)
            .first::<Question>(conn.deref())
            .map_err(Question::handle_error)?;
        let answers: Vec<Answer> = Answer::belonging_to(&question)
            .load::<Answer>(conn.deref())
            .map_err(Answer::handle_error)?;
        let user: User = users
            .find(question.author_id)
            .first::<User>(conn.deref())
            .map_err(User::handle_error)?;
        Ok((question, user, answers))
    }

    /// Gets a random question from the bucket that has not been answered yet.
    pub fn get_random_unanswered_question(bucket_id: i32, conn: &Conn) -> Result<(Question, User), WeekendAtJoesError> {
        use schema::users::dsl::*;

        let bucket = Bucket::get_bucket(bucket_id, &conn)?;
        let questions: Vec<Question> = Question::belonging_to(&bucket)
            .load::<Question>(conn.deref())
            .map_err(Question::handle_error)?;
        let answers: Vec<Answer> = Answer::belonging_to(&questions)
            .load::<Answer>(conn.deref())
            .map_err(Answer::handle_error)?;
        let grouped_answers: Vec<Vec<Answer>> = answers.grouped_by(&questions);

        // Select the questions that don't already have answers
        let unanswered_questions: Vec<Question> = questions
            .into_iter()
            .zip(grouped_answers)
            .filter(|x| x.1.len() == 0) // only keep the questions with unanswered questions
            .map(|x| x.0)
            .collect();

        // Select one random question from the group
        let mut rng = thread_rng();
        let random_question: Question = seq::sample_iter(&mut rng, unanswered_questions, 1)
            .map_err(|_| WeekendAtJoesError::InternalServerError)?
            .first()
            .cloned()
            .ok_or(WeekendAtJoesError::NotFound{ type_name: "Question" })?;

        // Get the matching user
        let user: User = users
            .find(random_question.author_id)
            .first::<User>(conn.deref())
            .map_err(User::handle_error)?; 

        Ok((random_question, user))
    }

    pub fn get_questions_for_bucket(owning_bucket_id: i32, conn: &Conn) -> Result<Vec< (User, Vec<(Question, Vec<Answer>)> )>, WeekendAtJoesError> {

        let bucket = Bucket::get_bucket(owning_bucket_id, &conn)?;
        let users: Vec<User> = User::get_all_users(conn)?;

        let questions: Vec<Question> = Question::belonging_to(&bucket)
            .load::<Question>(conn.deref())
            .map_err(Question::handle_error)?;
        let answers: Vec<Answer> = Answer::belonging_to(&questions)
            .load::<Answer>(conn.deref())
            .map_err(Answer::handle_error)?;
        let grouped_answers: Vec<Vec<Answer>> = answers.grouped_by(&questions);

        let questions_with_answers: Vec<Vec<(Question, Vec<Answer>)>> = questions
            .into_iter()
            .zip(grouped_answers)
            .grouped_by(&users);
        
        let retval: Vec< (User, Vec<(Question, Vec<Answer>)> )>  = users
            .into_iter()
            .zip(questions_with_answers)
            .collect();


        Ok(retval)
    }

    /// Gets a bucket by id.
    pub fn get_question(question_id: i32, conn: &Conn) -> Result<Question, WeekendAtJoesError> {
        use schema::questions::dsl::*;

        // Gets the first bucket that matches the id.
        questions
            .find(question_id)
            .first::<Question>(conn.deref())
            .map_err(Question::handle_error)

    }
}

impl ErrorFormatter for Question {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError {
        handle_diesel_error(diesel_error, "Question")
    }
}
