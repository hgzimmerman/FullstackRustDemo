use db::{
    bucket::Bucket,
    question::*,
};
use error::{
    Error,
    VectorMappable,
    *,
};
use identifiers::{
    bucket::BucketUuid,
    question::QuestionUuid,
};
use log::info;
use pool::Conn;
use rocket::Route;
use rocket_contrib::Json;
use routes::Routable;
use wire::question::*;

use auth_lib::user_authorization::NormalUser;

/// Get all questions in a given bucket.
#[get("/?<bucket_uuid>")]
fn get_questions_for_bucket(bucket_uuid: BucketUuid, conn: Conn) -> Result<Json<Vec<QuestionResponse>>, Error> {
    Question::get_questions_for_bucket(bucket_uuid, &conn)
        .map_vec::<QuestionResponse>()
        .map(Json)
}

/// Gets a random question from the bucket.
#[get("/random_question?<bucket_uuid>")]
fn get_random_question(bucket_uuid: BucketUuid, conn: Conn) -> Result<Json<QuestionResponse>, Error> {
    info!("Enter get random question");
    Question::get_random_question(bucket_uuid, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

/// Gets a question from the bucket by id.
#[get("/<question_uuid>")]
fn get_question(question_uuid: QuestionUuid, conn: Conn) -> Result<Json<QuestionResponse>, Error> {
    Question::get_full_question(question_uuid, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

/// Creates a question and puts it into the bucket.
/// Any user that belongs to the bucket can put a question into a bucket.
#[post("/create", data = "<new_question>")]
fn create_question(
    new_question: Json<NewQuestionRequest>,
    user: NormalUser,
    conn: Conn,
) -> Result<Json<QuestionResponse>, Error> {
    let request: NewQuestionRequest = new_question.into_inner();
    let bucket_uuid: BucketUuid = request.bucket_uuid;
    let is_approved = Bucket::is_user_approved(user.user_uuid, bucket_uuid, &conn);
    if !is_approved {
        return Err(Error::BadRequest);
    }

    let new_question: NewQuestion = NewQuestion::attach_user_id(request, user.user_uuid);

    Question::create_data(new_question, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

/// Permanently deletes the question from the database.
#[delete("/<question_uuid>")]
fn delete_question(question_uuid: QuestionUuid, user: NormalUser, conn: Conn) -> BackendResult<Json<QuestionUuid>> {
    info!(
        "user: {}, deleting question with id: {:?}",
        user.user_uuid, question_uuid
    );
    Question::delete_question(question_uuid.clone(), &conn)?; // spurious clone
    Ok(Json(question_uuid))
}

/// Takes a question that may have been on the floor and puts it back in the bucket.
#[put("/<question_uuid>/into_bucket")]
fn put_question_back_in_bucket(
    question_uuid: QuestionUuid,
    _user: NormalUser,
    conn: Conn,
) -> BackendResult<Json<QuestionUuid>> {
    Question::put_question_in_bucket(question_uuid, &conn)?;
    Ok(Json(question_uuid))
}

/// Gets the _number_ of questions that currently are in the bucket.
/// Being in the bucket is distinct from being on the floor.
/// Questions _in_ the bucket are eligible to be randomly selected, while those on the floor are not.
/// If the value returned by this endpoint is 0, then the client should not request random questions.
#[get("/quantity_in_bucket?<bucket_uuid>")]
fn questions_in_bucket(bucket_uuid: BucketUuid, _user: NormalUser, conn: Conn) -> BackendResult<Json<i64>> {
    Question::get_number_of_questions_in_bucket(bucket_uuid, &conn).map(Json)
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
