use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::message::*;
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::message::*;
use auth::user_authorization::NormalUser;
use db::Retrievable;
use db::Creatable;
use error::*;

impl From<MessageData> for MessageResponse {
    fn from(data: MessageData) -> MessageResponse {
        MessageResponse {
            id: data.message.id,
            author: data.author.into(),
            reply: data.reply
                .map(|x| MessageResponse::from(*x))
                .map(Box::new),
            content: data.message.message_content,
            date: data.message.create_date,
        }
    }
}



#[get("/<chat_id>")]
fn get_messages_for_chat(chat_id: i32, conn: Conn) -> JoeResult<Json<Vec<MessageResponse>>> {
    Message::get_messages_for_chat(chat_id, &conn)
        .map_vec::<MessageResponse>()
        .map(Json)

}


impl Routable for Message {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_messages_for_chat];
    const PATH: &'static str = "/message/";
}
