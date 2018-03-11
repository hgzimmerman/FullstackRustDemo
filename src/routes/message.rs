use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::message::*;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::message::*;
use auth::user_authorization::NormalUser;
use error::*;
use db::chat::Chat;




/// Gets messages for a given chat.
/// This will paginate, returning 25 messages at a time.
/// This operation is available to users who are part of the chat.
#[get("/<chat_id>/<index>")]
fn get_messages_for_chat(chat_id: i32, index: i32, user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<MessageResponse>>> {
    if !Chat::is_user_in_chat(chat_id, user.user_id, &conn)? {
        return Err(WeekendAtJoesError::BadRequest);
    }

    Message::get_messages_for_chat(chat_id, index, 25, &conn)
        .map_vec::<MessageResponse>()
        .map(Json)
}

#[post("/send", data = "<new_message>")]
fn send_message(new_message: Json<NewMessageRequest>, user: NormalUser, conn: Conn) -> JoeResult<Json<MessageResponse>> {
    let new_message: NewMessage = new_message.into_inner().into();

    if !Chat::is_user_in_chat(new_message.chat_id, user.user_id, &conn)? {
        return Err(WeekendAtJoesError::BadRequest);
    }
    if new_message.author_id != user.user_id {
        return Err(WeekendAtJoesError::BadRequest);
    }

    Message::create_message(new_message, &conn)
        .map(MessageResponse::from)
        .map(Json)
}


impl Routable for Message {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_messages_for_chat, send_message];
    const PATH: &'static str = "/message/";
}
