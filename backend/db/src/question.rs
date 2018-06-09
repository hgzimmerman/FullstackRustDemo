use schema::questions;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use user::User;
use bucket::Bucket;
use diesel::BelongingToDsl;
use answer::Answer;
use diesel::GroupedBy;
use answer::AnswerData;
use error::JoeResult;
use uuid::Uuid;
use identifiers::question::QuestionUuid;
use identifiers::bucket::BucketUuid;

use diesel;
use diesel::ExpressionMethods;
use diesel::PgConnection;

#[derive(Debug, Clone, Identifiable, Queryable, Associations, CrdUuid, ErrorHandler)]
#[primary_key(uuid)]
#[insertable = "NewQuestion"]
#[table_name = "questions"]
#[belongs_to(Bucket, foreign_key = "bucket_uuid")]
#[belongs_to(User, foreign_key = "author_uuid")]
pub struct Question {
    /// Primary Key.
    pub uuid: Uuid,
    pub bucket_uuid: Uuid,
    pub author_uuid: Uuid,
    pub question_text: String,
    pub on_floor: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub bucket_uuid: Uuid,
    pub author_uuid: Uuid,
    pub question_text: String,
    pub on_floor: bool, // Should be false by default
}

pub struct QuestionData {
    pub question: Question,
    pub user: User,
    pub answers: Vec<AnswerData>,
}

impl Question {
    /// Creates a new bucket
    pub fn create_data(new_question: NewQuestion, conn: &PgConnection) -> JoeResult<QuestionData> {
        let question: Question = Question::create(new_question, conn)?;
        let user = User::get_by_uuid(question.author_uuid, conn)?;

        Ok(QuestionData {
            question,
            user,
            answers: vec![],
        })

    }

    /// Gets a list of all questions across all buckets.
    pub fn get_questions(conn: &PgConnection) -> JoeResult<Vec<QuestionData>> {
        use schema::questions::dsl::*;
        use schema::users::dsl::*;
        let questions_and_users = questions
            .inner_join(users)
            .load::<(Question, User)>(conn)
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
    pub fn get_random_question(bucket_uuid: BucketUuid, conn: &PgConnection) -> JoeResult<QuestionData> {
        use schema::users::dsl::*;

        use schema::questions::columns::on_floor;

        // Get the bucket from which questions will be retrieved.
        let bucket = Bucket::get_by_uuid(bucket_uuid.0, &conn)?;

        no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

        // Get a random question belonging to the bucket.
        let question: Question = Question::belonging_to(&bucket)
            .order(RANDOM)
            .filter(on_floor.eq(false)) // Only get a question if it is not on the "floor" (and therefore in the bucket)
            .first::<Question>(conn)
            .map_err(Question::handle_error)?;
        // Get the answers associated with the question.
        let answers_and_users: Vec<(Answer, User)> = Answer::belonging_to(&question)
            .inner_join(users)
            .load::<(Answer, User)>(conn)
            .map_err(Answer::handle_error)?;
        // Get the author of the question.
        let user: User = users
            .find(question.author_uuid)
            .first::<User>(conn)
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

    // TODO, get rid of this.
    //    /// Gets a random question from the bucket that has not been answered yet.
    //    pub fn get_random_unanswered_question(bucket_id: i32, conn: &Conn) -> JoeResult<QuestionData> {
    //        use schema::users::dsl::*;
    //
    //        info!("Get Random Question: stop 0");
    //        // Get the bucket from which the questions will be retrieved.
    //        let bucket = Bucket::get_by_id(bucket_id, &conn)?;
    //
    //        info!("Get Random Question: stop 1");
    //        // Get all the questions in the bucket.
    //        let questions: Vec<Question> = Question::belonging_to(&bucket)
    //            .load::<Question>(conn.deref())
    //            .map_err(Question::handle_error)?;
    //
    //        info!("Get Random Question: stop 2");
    //        // Get all the answers belonging to the questions.
    //        let answers: Vec<Answer> = Answer::belonging_to(&questions)
    //            .load::<Answer>(conn.deref())
    //            .map_err(Answer::handle_error)?;
    //
    //        info!("Get Random Question: stop 3");
    //        // Group the answers in such a way that they correspond to their questions.
    //        let grouped_answers: Vec<Vec<Answer>> = answers.grouped_by(&questions);
    //
    //        info!("Get Random Question: stop 4");
    //        // Select the questions that don't already have answers
    //        let unanswered_questions: Vec<Question> = questions
    //            .into_iter()
    //            .zip(grouped_answers)
    //            .filter(|x| x.1.len() == 0) // only keep the questions with unanswered questions
    //            .map(|x| x.0) // only keep the questions
    //            .collect();
    //
    //        // Select one random question from the group
    //        let mut rng = thread_rng();
    //        let random_question: Question = seq::sample_iter(&mut rng, unanswered_questions, 1)
    //            .map_err(|_| WeekendAtJoesError::InternalServerError)?
    //            .first()
    //            .cloned()
    //            .ok_or(WeekendAtJoesError::NotFound { type_name: "Question" })?;
    //
    //        // Get the matching user
    //        let user: User = users
    //            .find(random_question.author_id)
    //            .first::<User>(conn.deref())
    //            .map_err(User::handle_error)?;
    //        Ok(QuestionData {
    //            question: random_question,
    //            user,
    //            answers: vec![],
    //        })
    //    }

    /// Gets groupings of questions, users, and answers for a given bucket id.
    pub fn get_questions_for_bucket(owning_bucket_uuid: BucketUuid, conn: &PgConnection) -> JoeResult<Vec<QuestionData>> {
        use schema::users::dsl::*;
        let bucket = Bucket::get_by_uuid(owning_bucket_uuid.0, &conn)?;

        let questions_and_users: Vec<(Question, User)> = Question::belonging_to(&bucket)
            .inner_join(users)
            .load::<(Question, User)>(conn)
            .map_err(Question::handle_error)?;

        let questions: Vec<Question> = questions_and_users
            .iter()
            .map(|q_and_u| q_and_u.0.clone())
            .collect();

        let answers: Vec<(Answer, User)> = Answer::belonging_to(&questions)
            .inner_join(users)
            .load::<(Answer, User)>(conn)
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

    /// The number corresponds to the number of questions that are eligable for selection via the random mechanic.
    /// This does not tightly correspond to the total number of questions associated with the bucket session.
    pub fn get_number_of_questions_in_bucket(bucket_uuid: BucketUuid, conn: &PgConnection) -> JoeResult<i64> {
        //        use schema::questions::dsl::*;
        use schema::questions;

        let bucket = Bucket::get_by_uuid(bucket_uuid.0, &conn)?;
        Question::belonging_to(&bucket)
            .filter(questions::on_floor.eq(false)) // if its not on the floor, it is in the bucket.
            .count()
            .get_result(conn)
            .map_err(Question::handle_error)
    }

    /// Given a question's id, get the question, its answers and user
    pub fn get_full_question(question_uuid: QuestionUuid, conn: &PgConnection) -> JoeResult<QuestionData> {
        use schema::users::dsl::*;

        // Get the question
        let question: Question = Question::get_by_uuid(question_uuid.0, conn)?;

        // Get the answers and their associated users and format them into answer data.
        let answer_data: Vec<AnswerData> = Answer::belonging_to(&question)
            .inner_join(users)
            .load::<(Answer, User)>(conn)
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
            .find(question.author_uuid)
            .first::<User>(conn)
            .map_err(User::handle_error)?;

        Ok(QuestionData {
            question,
            user,
            answers: answer_data,
        })
    }

    pub fn delete_question(question_uuid: QuestionUuid, conn: &PgConnection) -> JoeResult<Question> {
        let question_uuid = question_uuid.0;
        Question::delete_by_id(question_uuid, conn)
    }

    /// Puts the question in the metaphorical bucket, not the DB table.
    /// All this does is set a boolean indicating if the question is avalable for random selection or not.
    pub fn put_question_in_bucket(question_uuid: QuestionUuid, conn: &PgConnection) -> JoeResult<QuestionUuid> {
        use schema::questions::dsl::*;
        use schema::questions;


        let m_question_uuid: Uuid = question_uuid.0;

        let target = questions.filter(questions::uuid.eq(
            m_question_uuid,
        ));
        diesel::update(target)
            .set(on_floor.eq(false))
            .execute(conn)
            .map_err(Question::handle_error)?;
        Ok(question_uuid)
    }

    pub fn put_question_on_floor(question_uuid: QuestionUuid, conn: &PgConnection) -> JoeResult<QuestionUuid> {
        use schema::questions::dsl::*;
        use schema::questions;

        let m_question_uuid: Uuid = question_uuid.0;

        let target = questions.filter(questions::uuid.eq(
            m_question_uuid,
        ));
        diesel::update(target)
            .set(on_floor.eq(true))
            .execute(conn)
            .map_err(Question::handle_error)?;
        Ok(question_uuid)
    }
}
