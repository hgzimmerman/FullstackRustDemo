use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::question::*;
use error::WeekendAtJoesError;
use error::VectorMappable;
use db::Conn;
use wire::question::*;
use auth::user_authorization::*;

use error::*;
use log::info;


#[derive(FromForm)]
struct BucketIdParam {
    bucket_id: i32,
}

/// Get all questions in a given bucket.
#[get("/?<bucket_id_param>")]
fn get_questions_for_bucket(bucket_id_param: BucketIdParam, conn: Conn) -> Result<Json<Vec<QuestionResponse>>, WeekendAtJoesError> {

    Question::get_questions_for_bucket(bucket_id_param.bucket_id, &conn)
        .map_vec::<QuestionResponse>()
        .map(Json)
}

/// Gets a random question from the bucket.
#[get("/random_question?<bucket_id_param>")]
fn get_random_question(bucket_id_param: BucketIdParam, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    info!("Enter get random question");
    Question::get_random_question(bucket_id_param.bucket_id, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

/// Gets a question from the bucket by id.
#[get("/<question_id>")]
fn get_question(question_id: i32, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    Question::get_full_question(question_id, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

/// Creates a question and puts it into the bucket.
/// Any user can put a question into a bucket.
#[post("/create", data = "<new_question>")]
fn create_question(new_question: Json<NewQuestionRequest>, user: NormalUser, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    let request: NewQuestionRequest = new_question.into_inner();

    let new_question: NewQuestion = NewQuestion::attach_user_id(request, user.user_id);

    Question::create_data(new_question, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

/// Permanently deletes the question from the database.
#[delete("/<question_id>")]
fn delete_question(question_id: i32, user: NormalUser, conn: Conn) -> JoeResult<Json<i32>> {
    info!("user: {}, deleteting question with id: {}", user.user_id, question_id);
    Question::delete_question(question_id, &conn)?;
    Ok(Json(question_id))
}

/// Takes a question that may have been on the floor and puts it back in the bucket.
#[put("/<question_id>/into_bucket")]
fn put_question_back_in_bucket(question_id: i32, _user: NormalUser, conn: Conn) -> JoeResult<Json<i32>> {
    Question::put_question_in_bucket(question_id, &conn)?;
    Ok(Json(question_id))
}

/// Gets the _number_ of questions that currently are in the bucket.
/// Being in the bucket is distinct from being on the floor.
/// Questions _in_ the bucket are eligible to be randomly selected, while those on the floor are not.
/// If the value returned by this endpoint is 0, then the client should not request random questions.
#[get("/quantity_in_bucket?<bucket_id_param>")]
fn questions_in_bucket(bucket_id_param: BucketIdParam, _user: NormalUser, conn: Conn) -> JoeResult<Json<i64>> {
    Question::get_number_of_questions_in_bucket(bucket_id_param.bucket_id, &conn)
        .map(Json)
}

impl Routable for Question {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_question,
            get_random_question,
            get_questions_for_bucket,
            get_question,
            delete_question,
            put_question_back_in_bucket,
            questions_in_bucket,
        ]
    };
    const PATH: &'static str = "/question/";
}
