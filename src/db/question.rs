use schema::questions;
use error::*;
use db::Conn;
use std::ops::Deref;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use db::user::User;
use db::bucket::Bucket;
use diesel::BelongingToDsl;
use db::answer::Answer;
use diesel::GroupedBy;
use rand::{thread_rng, seq};
use db::answer::AnswerData;

#[derive(Debug, Clone, Identifiable, Queryable, Associations, Crd, ErrorHandler)]
#[insertable = "NewQuestion"]
#[table_name = "questions"]
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
#[table_name = "questions"]
pub struct NewQuestion {
    pub bucket_id: i32,
    pub author_id: i32,
    pub question_text: String,
}

pub struct QuestionData {
    pub question: Question,
    pub user: User,
    pub answers: Vec<AnswerData>,
}

impl Question {
    /// Creates a new bucket
    pub fn create_data(new_question: NewQuestion, conn: &Conn) -> JoeResult<QuestionData> {

        let question: Question = Question::create(new_question, conn)?;
        let user = User::get_by_id(question.author_id, conn)?;

        Ok(QuestionData {
            question,
            user,
            answers: vec![],
        })

    }

    /// Gets a list of all questions across all buckets.
    pub fn get_questions(conn: &Conn) -> JoeResult<Vec<QuestionData>> {
        use schema::questions::dsl::*;
        use schema::users::dsl::*;
        let questions_and_users = questions
            .inner_join(users)
            .load::<(Question, User)>(conn.deref())
            .map_err(Question::handle_error)?;

        let question_data: Vec<QuestionData> = questions_and_users
            .into_iter()
            .map(|x| {
                QuestionData {
                    question: x.0,
                    user: x.1,
                    answers: vec![], // TODO make a minimal response question
                }
            })
            .collect();
        Ok(question_data)
    }

    /// Gets a random question that may have already been answered
    pub fn get_random_question(bucket_id: i32, conn: &Conn) -> JoeResult<QuestionData> {
        use schema::users::dsl::*;

        // Get the bucket from which questions will be retrieved.
        let bucket = Bucket::get_by_id(bucket_id, &conn)?;

        no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

        // Get a random question belonging to the bucket.
        let question: Question = Question::belonging_to(&bucket)
            .order(RANDOM)
            .first::<Question>(conn.deref())
            .map_err(Question::handle_error)?;
        // Get the answers associated with the question.
        let answers_and_users: Vec<(Answer, User)> = Answer::belonging_to(&question)
            .inner_join(users)
            .load::<(Answer, User)>(conn.deref())
            .map_err(Answer::handle_error)?;
        // Get the author of the question.
        let user: User = users
            .find(question.author_id)
            .first::<User>(conn.deref())
            .map_err(User::handle_error)?;
        // Get them all together.

        Ok(QuestionData {
            question,
            user,
            answers: answers_and_users
                .into_iter()
                .map(|x| {
                    AnswerData {
                        answer: x.0,
                        user: x.1,
                    }
                })
                .collect(),
        })
    }

    /// Gets a random question from the bucket that has not been answered yet.
    pub fn get_random_unanswered_question(bucket_id: i32, conn: &Conn) -> JoeResult<QuestionData> {
        use schema::users::dsl::*;

        // Get the bucket from which the questions will be retrieved.
        let bucket = Bucket::get_by_id(bucket_id, &conn)?;
        // Get all the questions in the bucket.
        let questions: Vec<Question> = Question::belonging_to(&bucket)
            .load::<Question>(conn.deref())
            .map_err(Question::handle_error)?;
        // Get all the answers belonging to the questions.
        let answers: Vec<Answer> = Answer::belonging_to(&questions)
            .load::<Answer>(conn.deref())
            .map_err(Answer::handle_error)?;
        // Group the answers in such a way that they correspond to their questions.
        let grouped_answers: Vec<Vec<Answer>> = answers.grouped_by(&questions);

        // Select the questions that don't already have answers
        let unanswered_questions: Vec<Question> = questions
            .into_iter()
            .zip(grouped_answers)
            .filter(|x| x.1.len() == 0) // only keep the questions with unanswered questions
            .map(|x| x.0) // only keep the questions
            .collect();

        // Select one random question from the group
        let mut rng = thread_rng();
        let random_question: Question = seq::sample_iter(&mut rng, unanswered_questions, 1)
            .map_err(|_| WeekendAtJoesError::InternalServerError)?
            .first()
            .cloned()
            .ok_or(WeekendAtJoesError::NotFound { type_name: "Question" })?;

        // Get the matching user
        let user: User = users
            .find(random_question.author_id)
            .first::<User>(conn.deref())
            .map_err(User::handle_error)?;
        Ok(QuestionData {
            question: random_question,
            user,
            answers: vec![],
        })
    }

    /// Gets groupings of questions, users, and answers for a given bucket id.
    pub fn get_questions_for_bucket(owning_bucket_id: i32, conn: &Conn) -> JoeResult<Vec<QuestionData>> {
        use schema::users::dsl::*;
        let bucket = Bucket::get_by_id(owning_bucket_id, &conn)?;

        let questions_and_users: Vec<(Question, User)> = Question::belonging_to(&bucket)
            .inner_join(users)
            .load::<(Question, User)>(conn.deref())
            .map_err(Question::handle_error)?;

        let questions: Vec<Question> = questions_and_users
            .iter()
            .map(|q_and_u| q_and_u.0.clone())
            .collect();

        let answers: Vec<(Answer, User)> = Answer::belonging_to(&questions)
            .inner_join(users)
            .load::<(Answer, User)>(conn.deref())
            .map_err(Answer::handle_error)?;
        let grouped_answers: Vec<Vec<(Answer, User)>> = answers.grouped_by(&questions); // I'm not 100% shure that this works as intended here

        let data_tuple: Vec<((Question, User), Vec<(Answer, User)>)> = questions_and_users
            .into_iter()
            .zip(grouped_answers)
            .collect();

        let question_data = data_tuple
            .into_iter()
            .map(|x| {
                let question = (x.0).0;
                let user = (x.0).1;
                let a_u = x.1;
                QuestionData {
                    question,
                    user,
                    answers: a_u.into_iter()
                        .map(|y| {
                            AnswerData {
                                answer: y.0,
                                user: y.1,
                            }
                        })
                        .collect(),
                }
            })
            .collect();
        Ok(question_data)
    }


    /// Given a question's id, get the question, its answers and user
    pub fn get_full_question(q_id: i32, conn: &Conn) -> JoeResult<QuestionData> {
        use schema::users::dsl::*;

        // Get the question
        let question: Question = Question::get_by_id(q_id, conn)?;

        // Get the answers and their associated users and format them into answer data.
        let answer_data: Vec<AnswerData> = Answer::belonging_to(&question)
            .inner_join(users)
            .load::<(Answer, User)>(conn.deref())
            .map_err(Answer::handle_error)?
            .into_iter()
            .map(|x| {
                AnswerData {
                    answer: x.0,
                    user: x.1,
                }
            })
            .collect();

        // Get the matching user
        let user: User = users
            .find(question.author_id)
            .first::<User>(conn.deref())
            .map_err(User::handle_error)?;

        Ok(QuestionData {
            question,
            user,
            answers: answer_data
        })
    }
}
