use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::answer::*;
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use wire::answer::*;
use auth_lib::user_authorization::NormalUser;

use db::question::Question;
use identifiers::question::QuestionUuid;
use identifiers::user::UserUuid;


/// Answers a bucket question by attaching the answer to the existing question.
/// This will also remove the question from consideration when randomly selecting questions (putting it on the floor).
/// This operation is available to any user.
#[post("/create", data = "<new_answer>")]
fn answer_question(new_answer: Json<NewAnswerRequest>, user: NormalUser, conn: Conn) -> Result<Json<AnswerResponse>, WeekendAtJoesError> {
    let new_answer: NewAnswerRequest = new_answer.into_inner();
    let question_uuid: QuestionUuid = new_answer.question_uuid.clone(); // spurious clone

    let new_answer: NewAnswer = NewAnswer::attach_user_id(new_answer, user.user_uuid);
    let author_uuid = UserUuid(new_answer.author_uuid);
    let answer_user: User = User::get_user(author_uuid, &conn)?;


    Question::put_question_on_floor(question_uuid, &conn)?;

    Answer::create_answer(new_answer, &conn)
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
