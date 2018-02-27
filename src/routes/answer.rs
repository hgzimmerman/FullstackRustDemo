use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::answer::*;
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::answer::*;
use auth::user_authorization::NormalUser;


pub struct AnswerData(pub (Answer, User));
impl From<AnswerData> for AnswerResponse {
    fn from(answer_data: AnswerData ) -> AnswerResponse {
        let (answer, user) = answer_data.0;
        AnswerResponse {
            id: answer.id,
            answer_text: answer.answer_text,
            author: user.into(),
        }
    }
}

impl From<NewAnswerRequest> for NewAnswer {
    fn from(request: NewAnswerRequest) -> NewAnswer {
        NewAnswer {
            answer_text: request.answer_text,
            author_id: request.author_id,
            question_id: request.question_id
        }
    }
}


#[post("/create", data = "<new_answer>")]
fn answer_question(new_answer: Json<NewAnswerRequest>, _user: NormalUser, conn: Conn) -> Result<Json<AnswerResponse>, WeekendAtJoesError> {
    let new_answer: NewAnswer = new_answer.into_inner().into();
    let user: User = User::get_user(new_answer.author_id, &conn)?;

    Answer::create_answer(new_answer, &conn)
        .map(|x| AnswerData((x, user)))
        .map(AnswerResponse::from)
        .map(Json)

    // Answer::create_answer()
}

impl Routable for Answer {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![
        answer_question,
    ];
    const PATH: &'static str = "/answer/";
}