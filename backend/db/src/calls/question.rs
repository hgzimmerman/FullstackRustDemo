use crate::{
    answer::{
        Answer,
        AnswerData,
    },
    bucket::Bucket,
    calls::prelude::*,
    schema::{
        self,
        questions,
        junction_favorite_questions_users
    },
    user::User,
};
use diesel::{
    self,
    BelongingToDsl,
    ExpressionMethods,
    GroupedBy,
    PgConnection,
    QueryDsl,
    RunQueryDsl,
};
use error::BackendResult;
use identifiers::{
    bucket::BucketUuid,
    question::QuestionUuid,
    user::UserUuid,
};
use uuid::Uuid;
use diesel::pg::expression::array_comparison::any;

#[derive(Debug, Clone, Identifiable, Queryable, Associations, TypeName)]
#[primary_key(uuid)]
#[table_name = "questions"]
#[belongs_to(Bucket, foreign_key = "bucket_uuid")]
#[belongs_to(User, foreign_key = "author_uuid")]
pub struct Question {
    /// Primary Key.
    pub uuid: Uuid,
    pub bucket_uuid: Uuid,
    pub author_uuid: Option<Uuid>,
    pub question_text: String,
    pub on_floor: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub bucket_uuid: Uuid,
    pub author_uuid: Option<Uuid>,
    pub question_text: String,
    pub on_floor: bool, // Should be false by default
}

#[derive(Debug)]
pub struct QuestionData {
    pub question: Question,
    pub user: Option<User>,
    pub answers: Vec<AnswerData>,
}

#[derive(Debug, Clone, Identifiable, Queryable)]
#[table_name = "junction_favorite_questions_users"]
#[primary_key(uuid)]
pub struct FavoriteQuestionJunction {
    pub uuid: Uuid,
    pub question_uuid: Uuid,
    pub user_uuid: Uuid,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "junction_favorite_questions_users"]
pub struct NewFavoriteQuestionJunction {
    pub question_uuid: Uuid,
    pub user_uuid: Uuid,
}

impl Question {
    pub fn get_question(uuid: QuestionUuid, conn: &PgConnection) -> BackendResult<Question> {
        get_row::<Question, _>(schema::questions::table, uuid.0, conn)
    }
    pub fn delete_question(uuid: QuestionUuid, conn: &PgConnection) -> BackendResult<Question> {
        delete_row::<Question, _>(schema::questions::table, uuid.0, conn)
    }
    pub fn create_question(new: NewQuestion, conn: &PgConnection) -> BackendResult<Question> {
        create_row::<Question, NewQuestion, _>(schema::questions::table, new, conn)
    }

    /// Creates a new bucket
    pub fn create_data(new_question: NewQuestion, conn: &PgConnection) -> BackendResult<QuestionData> {
        let question: Question = Question::create_question(new_question, conn)?;

        let user: Option<User> =  question.author_uuid
            .map(UserUuid)
            .map(|author_uuid| {
                User::get_user(author_uuid, conn)
            })
            .transpose()?;

        Ok(QuestionData {
            question,
            user,
            answers: vec![],
        })
    }

    /// Gets a list of all questions across all buckets.
    pub fn get_questions(conn: &PgConnection) -> BackendResult<Vec<QuestionData>> {
        use crate::schema::{
            questions::dsl::*,
            users::dsl::*,
        };
        let questions_and_users: Vec<(Question, Option<User>)> = questions
            .left_join(users) // TODO investigate this join
            .load::<(Question, Option<User>)>(conn)
            .map_err(handle_err::<Question>)?;

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
    pub fn get_random_question(bucket_uuid: BucketUuid, conn: &PgConnection) -> BackendResult<QuestionData> {
        use crate::schema::users::dsl::*;

        use crate::schema::questions::columns::on_floor;

        // Get the bucket from which questions will be retrieved.
        let bucket = Bucket::get_bucket(bucket_uuid, &conn)?;

        no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

        // Get a random question belonging to the bucket.
        let question: Question = Question::belonging_to(&bucket)
            .order(RANDOM)
            .filter(on_floor.eq(false)) // Only get a question if it is not on the "floor" (and therefore in the bucket)
            .first::<Question>(conn)
            .map_err(handle_err::<Question>)?;
        // Get the answers associated with the question.
        let answers_and_users: Vec<(Answer, Option<User>)> = Answer::belonging_to(&question)
            .left_join(users)
            .load::<(Answer, Option<User>)>(conn)
            .map_err(handle_err::<Answer>)?;
        // Get the author of the question.
        let user: Option<User> = if let Some(author_uuid) = question.author_uuid {
            let author = users
                .find(author_uuid)
                .first::<User>(conn)
                .map_err(handle_err::<User>)?;
            Some(author)
        } else {
            None
        };


        // Get them all together.

        Ok(QuestionData {
            question,
            user,
            answers: answers_and_users
                .into_iter()
                .map(|x| AnswerData { answer: x.0, user: x.1 })
                .collect(),
        })
    }

    /// Helper function that gets answers for a set of questions and properly spaces them so that they may be joined together.
    fn get_grouped_answers_for_questions(questions: &[Question], conn: &PgConnection) -> BackendResult<Vec<Vec<(Answer, Option<User>)>>> {
        use crate::schema::users::dsl::*;
        let answers: Vec<(Answer, Option<User>)> = Answer::belonging_to(questions)
            .left_join(users)
            .load::<(Answer, Option<User>)>(conn)
            .map_err(handle_err::<Answer>)?;
        let grouped_answers: Vec<Vec<(Answer, Option<User>)>> = answers.grouped_by(questions);

        Ok(grouped_answers)

    }

    /// Helper function that gets answers and associates that data with the provided questions.
    fn get_answers_for_questions_and_users(questions_and_users: Vec<(Question, Option<User>)>, conn: &PgConnection) -> BackendResult<Vec<QuestionData>> {
        let questions: Vec<Question> = questions_and_users.iter().map(|q_and_u| q_and_u.0.clone()).collect();
        let grouped_answers = Self::get_grouped_answers_for_questions(&questions, conn)?;

        let data_tuple: Vec<((Question, Option<User>), Vec<(Answer, Option<User>)>)> =
            questions_and_users.into_iter().zip(grouped_answers).collect();

        // Convert to the question data
        let question_data = data_tuple
            .into_iter()
            .map(|x| {
                let question = (x.0).0;
                let user = (x.0).1;
                let a_u = x.1;
                QuestionData {
                    question,
                    user,
                    answers: a_u.into_iter().map(|y| AnswerData { answer: y.0, user: y.1 }).collect(),
                }
            })
            .collect();
        Ok(question_data)
    }


    /// Gets groupings of questions, users, and answers for a given bucket id.
    pub fn get_questions_for_bucket(
        owning_bucket_uuid: BucketUuid,
        conn: &PgConnection,
    ) -> BackendResult<Vec<QuestionData>> {
        use crate::schema::users::dsl::*;
        let bucket = Bucket::get_bucket(owning_bucket_uuid, &conn)?;

        let questions_and_users: Vec<(Question, Option<User>)> = Question::belonging_to(&bucket)
            .left_join(users)
            .load::<(Question, Option<User>)>(conn)
            .map_err(handle_err::<Question>)?;


        Self::get_answers_for_questions_and_users(questions_and_users, conn)

    }

    /// The number corresponds to the number of questions that are eligable for selection via the random mechanic.
    /// This does not tightly correspond to the total number of questions associated with the bucket session.
    pub fn get_number_of_questions_in_bucket(bucket_uuid: BucketUuid, conn: &PgConnection) -> BackendResult<i64> {
        //        use schema::questions::dsl::*;
        use crate::schema::questions;

        let bucket = Bucket::get_bucket(bucket_uuid, &conn)?;
        Question::belonging_to(&bucket)
            .filter(questions::on_floor.eq(false)) // if its not on the floor, it is in the bucket.
            .count()
            .get_result(conn)
            .map_err(handle_err::<Question>)
    }

    /// Given a question's id, get the question, its answers and user
    pub fn get_full_question(question_uuid: QuestionUuid, conn: &PgConnection) -> BackendResult<QuestionData> {
        use crate::schema::users::dsl::*;

        // Get the question
        let question: Question = Question::get_question(question_uuid, conn)?;

        let to_answer_data = |x: (Answer, Option<User>)| AnswerData { answer: x.0, user: x.1 };

        // Get the answers and their associated users and format them into answer data.
        let answer_data: Vec<AnswerData> = Answer::belonging_to(&question)
            .left_join(users)
            .load::<(Answer, Option<User>)>(conn)
            .map_err(handle_err::<Answer>)?
            .into_iter()
            .map(to_answer_data)
            .collect();

        // Get the matching user
        let user: Option<User> = if let Some(author_uuid) = question.author_uuid {
            let user = users
                .find(author_uuid)
                .first::<User>(conn)
                .map_err(handle_err::<User>)?;
            Some(user)
        } else {
            None
        };

        Ok(QuestionData {
            question,
            user,
            answers: answer_data,
        })
    }

    //    pub fn delete_question(question_uuid: QuestionUuid, conn: &PgConnection) -> JoeResult<Question> {
    //        let question_uuid = question_uuid.0;
    //        Question::delete_by_id(question_uuid, conn)
    //    }

    /// Marks a question as a favorite for a given user.
    pub fn favorite_question(question_uuid: QuestionUuid, user_uuid: UserUuid, conn: &PgConnection) -> BackendResult<()> {
        use crate::schema::junction_favorite_questions_users as favorites;
        let junction_record = NewFavoriteQuestionJunction {
            question_uuid: question_uuid.0,
            user_uuid: user_uuid.0
        };

        diesel::insert_into(favorites::table)
            .values(junction_record)
            .execute(conn)
            .map_err(handle_err::<Question>)?;
        Ok(())
    }

    /// Unmarks a question as a favorite for a given user.
    pub fn unfavorite_question(question_uuid: QuestionUuid, user_uuid: UserUuid, conn: &PgConnection) -> BackendResult<()> {
        use crate::schema::junction_favorite_questions_users as favorites;

        diesel::delete(favorites::table)
            .filter(favorites::question_uuid.eq(question_uuid.0))
            .filter(favorites::user_uuid.eq(user_uuid.0))
            .execute(conn)
            .map_err(handle_err::<Question>)?;
        Ok(())
    }

    /// Gets all of the questions a user has favorited.
    pub fn get_favorite_questions(user_uuid: UserUuid, conn: &PgConnection) -> BackendResult<Vec<QuestionData>> {
        use crate::schema::users::dsl::*;
        use crate::schema::junction_favorite_questions_users as favorites;
        use crate::schema::questions;

        let question_uuids: Vec<Uuid> =
           favorites::table
                .filter(favorites::user_uuid.eq(user_uuid.0))
                .select(favorites::question_uuid)
                .load::<Uuid>(conn)
                .map_err(handle_err::<User>)?;

           let questions_and_users = questions::table
                .filter(questions::uuid.eq(any(question_uuids)))
                .left_join(users)
                .load::<(Question, Option<User>)>(conn)
                .map_err(handle_err::<User>)?;
        Self::get_answers_for_questions_and_users(questions_and_users, conn)
    }

    /// Puts the question in the metaphorical bucket, not the DB table.
    /// All this does is set a boolean indicating if the question is avalable for random selection or not.
    pub fn put_question_in_bucket(question_uuid: QuestionUuid, conn: &PgConnection) -> BackendResult<QuestionUuid> {
        use crate::schema::questions::{
            self,
            dsl::*,
        };

        let m_question_uuid: Uuid = question_uuid.0;

        let target = questions.filter(questions::uuid.eq(m_question_uuid));
        diesel::update(target)
            .set(on_floor.eq(false))
            .execute(conn)
            .map_err(handle_err::<Question>)?;
        Ok(question_uuid)
    }

    pub fn put_question_on_floor(question_uuid: QuestionUuid, conn: &PgConnection) -> BackendResult<QuestionUuid> {
        use crate::schema::questions::{
            self,
            dsl::*,
        };

        let m_question_uuid: Uuid = question_uuid.0;

        let target = questions.filter(questions::uuid.eq(m_question_uuid));
        diesel::update(target)
            .set(on_floor.eq(true))
            .execute(conn)
            .map_err(handle_err::<Question>)?;
        Ok(question_uuid)
    }
}
