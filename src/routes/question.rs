use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::question::*;
use error::WeekendAtJoesError;
use error::VectorMappable;
use db::Conn;
use requests_and_responses::question::*;
use auth::user_authorization::*;






/// Get all questions in a given bucket.
#[get("/questions_in_bucket/<bucket_id>")]
fn get_questions_for_bucket(bucket_id: i32, conn: Conn) -> Result<Json<Vec<QuestionResponse>>, WeekendAtJoesError> {

    Question::get_questions_for_bucket(bucket_id, &conn)
        .map_vec::<QuestionResponse>()
        .map(Json)
}

/// Gets a random question from the bucket.
#[get("/random_question/<bucket_id>")]
fn get_random_unanswered_question(bucket_id: i32, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    Question::get_random_unanswered_question(bucket_id, &conn)
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
    if request.author_id != user.user_id {
        return Err(WeekendAtJoesError::BadRequest);
    }
    Question::create_data(request.into(), &conn)
        .map(QuestionResponse::from)
        .map(Json)
}



impl Routable for Question {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_question,
            get_random_unanswered_question,
            get_questions_for_bucket,
            get_question,
        ]
    };
    const PATH: &'static str = "/question/";
}
