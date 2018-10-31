use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;
use db::message::*;
use error::Error;
use db::Conn;
use wire::message::*;
use auth_lib::user_authorization::NormalUser;
use error::*;
use db::chat::Chat;
use identifiers::chat::ChatUuid;



// TODO this API route needs some work
/// Gets messages for a given chat.
/// This will paginate, returning 25 messages at a time.
/// This operation is available to users who are part of the chat.
#[get("/<index>?<chat_uuid>")]
fn get_messages_for_chat(chat_uuid: ChatUuid, index: i32, user: NormalUser, conn: Conn) -> BackendResult<Json<Vec<MessageResponse>>> {
    if !Chat::is_user_in_chat(&chat_uuid, user.user_uuid, &conn)? {
        return Err(Error::BadRequest);
    }

    Message::get_messages_for_chat(chat_uuid, index, 25, &conn)
        .map_vec::<MessageResponse>()
        .map(Json)
}

/// Sends a message to the group.
#[post("/send", data = "<new_message>")]
fn send_message(new_message: Json<NewMessageRequest>, user: NormalUser, conn: Conn) -> BackendResult<Json<MessageResponse>> {

    if !Chat::is_user_in_chat(&new_message.chat_uuid, user.user_uuid, &conn)? {
        return Err(Error::BadRequest);
    }
    if new_message.author_uuid != user.user_uuid {
        return Err(Error::BadRequest);
    }

    let new_message: NewMessage = new_message.into_inner().into();
    Message::create_message(new_message, &conn)
        .map(MessageResponse::from)
        .map(Json)
}


impl Routable for Message {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_messages_for_chat, send_message];
    const PATH: &'static str = "/message/";
}
