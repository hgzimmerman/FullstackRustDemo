use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::answer::*;
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use wire::answer::*;
use auth::user_authorization::NormalUser;
use db::Retrievable;
use db::CreatableUuid;
use db::question::Question;
use identifiers::question::QuestionUuid;



/// Answers a bucket question by attaching the answer to the existing question.
/// This will also remove the question from consideration when randomly selecting questions (putting it on the floor).
/// This operation is available to any user.
#[post("/create", data = "<new_answer>")]
fn answer_question(new_answer: Json<NewAnswerRequest>, user: NormalUser, conn: Conn) -> Result<Json<AnswerResponse>, WeekendAtJoesError> {
    let new_answer: NewAnswerRequest = new_answer.into_inner();
    let question_uuid: QuestionUuid = new_answer.question_id.clone(); // spurious clone

    let new_answer: NewAnswer = NewAnswer::attach_user_id(new_answer, user.user_id);
    let answer_user: User = User::get_by_id(new_answer.author_id, &conn)?;


    Question::put_question_on_floor(question_uuid, &conn)?;

    Answer::create(new_answer, &conn)
        .map(|answer| {
            AnswerData {
                answer,
                user: answer_user,
            }
        })
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
