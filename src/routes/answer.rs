use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::answer::*;
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::answer::*;
use auth::user_authorization::NormalUser;
use db::Retrievable;
use db::Creatable;



/// Answers a bucket question by attaching the answer to the existing question.
/// This operation is available to any user.
#[post("/create", data = "<new_answer>")]
fn answer_question(new_answer: Json<NewAnswerRequest>, _user: NormalUser, conn: Conn) -> Result<Json<AnswerResponse>, WeekendAtJoesError> {
    let new_answer: NewAnswer = new_answer.into_inner().into();
    let user: User = User::get_by_id(new_answer.author_id, &conn)?;

    Answer::create(new_answer, &conn)
        .map(|answer| AnswerData { answer, user })
        .map(AnswerResponse::from)
        .map(Json)

}

impl Routable for Answer {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
        answer_question,
    ]
    };
    const PATH: &'static str = "/answer/";
}
