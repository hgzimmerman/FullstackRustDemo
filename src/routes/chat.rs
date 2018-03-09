use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::chat::Chat;
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::chat::*;
use auth::user_authorization::NormalUser;
use db::Retrievable;
use db::Creatable;
use error::JoeResult;




// impl From<AnswerData> for AnswerResponse {
//     fn from(data: AnswerData) -> AnswerResponse {
//         AnswerResponse {
//             id: data.answer.id,
//             answer_text: data.answer.answer_text,
//             author: data.user.into(),
//         }
//     }
// }

// impl From<NewAnswerRequest> for NewAnswer {
//     fn from(request: NewAnswerRequest) -> NewAnswer {
//         NewAnswer {
//             answer_text: request.answer_text,
//             author_id: request.author_id,
//             question_id: request.question_id,
//         }
//     }
// }


#[post("/create", data = "<new_chat>")]
fn create_chat(new_chat: Json<NewChatRequest>, _user: NormalUser, conn: Conn) -> JoeResult<Json<ChatResponse>> {
    // let new_answer: NewAnswer = new_answer.into_inner().into();
    // let user: User = User::get_by_id(new_answer.author_id, &conn)?;

    // Answer::create(new_answer, &conn)
    //     .map(|answer| AnswerData { answer, user })
    //     .map(AnswerResponse::from)
    //     .map(Json)
    unimplemented!()
}

fn add_user_to_chat(_user: NormalUser, conn: Conn) -> JoeResult<Json<ChatResponse>> {
    unimplemented!()
}

fn remove_user_from_chat(_user: NormalUser, conn: Conn) -> JoeResult<Json<ChatResponse>> {
    unimplemented!()
}

impl Routable for Chat {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
    ]
    };
    const PATH: &'static str = "/chat/";
}
