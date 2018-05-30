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
use log;


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

// TODO use a query parameter
/// Gets a random question from the bucket.
#[get("/random_question?<bucket_id_param>")]
fn get_random_question(bucket_id_param: BucketIdParam, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    log::info!("Enter get random question");
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

#[delete("/<question_id>")]
fn delete_question(question_id: i32, user: NormalUser, conn: Conn) -> JoeResult<Json<i32>> {
    log::info!("user: {}, deleteting question with id: {}", user.user_id, question_id);
    Question::delete_question(question_id, &conn)?;
    Ok(Json(question_id))
}

#[put("/<question_id>/into_bucket")]
fn put_question_back_in_bucket(question_id: i32, _user: NormalUser, conn: Conn) -> JoeResult<Json<i32>> {
    Question::put_question_in_bucket(question_id, &conn)?;
    Ok(Json(question_id))
}

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
            questions_in_bucket
        ]
    };
    const PATH: &'static str = "/question/";
}
