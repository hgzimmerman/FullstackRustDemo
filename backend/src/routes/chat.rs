use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::chat::*;
use db::Conn;
use wire::chat::*;
use auth::user_authorization::NormalUser;
use error::JoeResult;
use db::Creatable;
use error::*;
use db::Retrievable;

use log;


/// Creates a new chat.
/// This operation is available to any user.
#[post("/create", data = "<new_chat>")]
fn create_chat(new_chat: Json<NewChatRequest>, user: NormalUser, conn: Conn) -> JoeResult<Json<MinimalChatResponse>> {

    let new_chat: NewChat = new_chat.into_inner().into();

    if new_chat.leader_id != user.user_id {
        log::info!("User tried to create a chat where they are not the leader");
        return Err(WeekendAtJoesError::BadRequest);
    }

    Chat::create(new_chat, &conn)
        .map(MinimalChatResponse::from)
        .map(Json)
}

/// Adds the user to the chat.
/// This operation is available to any user.
#[put("/add_user", data = "<request>")]
fn add_user_to_chat(request: Json<ChatUserAssociationRequest>, user: NormalUser, conn: Conn) -> JoeResult<Json<()>> {

    let association: ChatUserAssociation = request.into_inner().into();

    if !Chat::is_user_in_chat(association.chat_id, user.user_id, &conn)? {
        log::info!("User not in a chat tried to add a user to that chat.");
        return Err(WeekendAtJoesError::BadRequest);
    }


    Chat::add_user_to_chat(association, &conn)
        .map(Json)
}

/// Removes the user from the chat.
/// This operation is available to any user.
#[put("/remove_user", data = "<request>")]
fn remove_user_from_chat(request: Json<ChatUserAssociationRequest>, user: NormalUser, conn: Conn) -> JoeResult<Json<()>> {

    let association: ChatUserAssociation = request.into_inner().into();
    let chat: Chat = Chat::get_by_id(association.chat_id, &conn)?;

    if chat.leader_id != user.user_id {
        log::info!("User without chat leader status tried to remove user");
        return Err(WeekendAtJoesError::BadRequest);
    }

    Chat::remove_user_from_chat(association, &conn)
        .map(Json)
}

/// Gets all of the chats (name and Id) that are associated with the user.
#[get("/belonging_to_user")]
fn get_chats_for_user(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<MinimalChatResponse>>> {
    Chat::get_chats_user_is_in(user.user_id, &conn)
        .map_vec::<MinimalChatResponse>()
        .map(Json)
}

/// Gets the full details of a chat for a user.
#[get("/<chat_id>")]
fn get_chat(chat_id: i32, _user: NormalUser, conn: Conn) -> JoeResult<Json<ChatResponse>> {
    Chat::get_full_chat(chat_id, &conn)
        .map(ChatResponse::from)
        .map(Json)

}

impl Routable for Chat {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_chat,
            add_user_to_chat,
            remove_user_from_chat,
            get_chats_for_user,
            get_chat,
        ]
    };
    const PATH: &'static str = "/chat/";
}
