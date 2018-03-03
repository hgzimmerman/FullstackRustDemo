use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::question::*;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::question::*;
use requests_and_responses::answer::*;
use auth::user_authorization::*;




impl From<QuestionData> for QuestionResponse {
    fn from(data: QuestionData) -> QuestionResponse {

        QuestionResponse {
            id: data.question.id,
            bucket_id: data.question.bucket_id,
            question_text: data.question.question_text,
            author: data.user.clone().into(),
            answers: data.answers
                .into_iter()
                .map(AnswerResponse::from)
                .collect(),
        }
    }
}

impl From<NewQuestionRequest> for NewQuestion {
    fn from(request: NewQuestionRequest) -> NewQuestion {
        NewQuestion {
            bucket_id: request.bucket_id,
            author_id: request.author_id,
            question_text: request.question_text,
        }
    }
}

#[get("/questions_in_bucket/<bucket_id>")]
fn get_questions_for_bucket(bucket_id: i32, conn: Conn) -> Result<Json<Vec<QuestionResponse>>, WeekendAtJoesError> {

    Question::get_questions_for_bucket(bucket_id, &conn)
        .map(|questions| {
            questions
                .into_iter()
                .map(QuestionResponse::from)
                .collect()
        })
        .map(Json)
}

#[get("/random_question/<bucket_id>")]
fn get_random_unanswered_question(bucket_id: i32, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    Question::get_random_unanswered_question(bucket_id, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

#[get("/<question_id>")]
fn get_question(question_id: i32, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    Question::get_full_question(question_id, &conn)
        .map(QuestionResponse::from)
        .map(Json)
}

#[post("/create", data = "<new_question>")]
fn create_question(new_question: Json<NewQuestionRequest>, _user: NormalUser, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    let request: NewQuestionRequest = new_question.into_inner();
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
