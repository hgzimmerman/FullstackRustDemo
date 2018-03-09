use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::chat::*;
// use db::user::User;
// use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::chat::*;
use requests_and_responses::user::UserResponse;
use auth::user_authorization::NormalUser;
use error::JoeResult;
use db::Creatable;
use error::*;



impl From<ChatUserAssociationRequest> for ChatUserAssociation {
    fn from(request: ChatUserAssociationRequest) -> ChatUserAssociation {
        ChatUserAssociation {
            user_id: request.user_id,
            chat_id: request.chat_id,
        }
    }
}

impl From<ChatData> for ChatResponse {
    fn from(data: ChatData) -> ChatResponse {
        ChatResponse {
            id: data.chat.id,
            name: data.chat.chat_name,
            leader: data.leader.into(),
            members: data.members
                .into_iter()
                .map(UserResponse::from)
                .collect(),
        }
    }
}

impl From<Chat> for MinimalChatResponse {
    fn from(chat: Chat) -> MinimalChatResponse {
        MinimalChatResponse {
            id: chat.id,
            name: chat.chat_name,
        }
    }
}

impl From<NewChatRequest> for NewChat {
    fn from(request: NewChatRequest) -> NewChat {
        NewChat {
            chat_name: request.name,
            leader_id: request.leader_id,
        }
    }
}



#[post("/create", data = "<new_chat>")]
fn create_chat(new_chat: Json<NewChatRequest>, _user: NormalUser, conn: Conn) -> JoeResult<Json<MinimalChatResponse>> {
    Chat::create(new_chat.into_inner().into(), &conn)
        .map(MinimalChatResponse::from)
        .map(Json)
}

#[put("/add_user", data = "<request>")]
fn add_user_to_chat(request: Json<ChatUserAssociationRequest>, _user: NormalUser, conn: Conn) -> JoeResult<Json<()>> {
    Chat::add_user_to_chat(request.into_inner().into(), &conn)
        .map(Json)
}

#[put("/remove_user", data = "<request>")]
fn remove_user_from_chat(request: Json<ChatUserAssociationRequest>, _user: NormalUser, conn: Conn) -> JoeResult<Json<()>> {
    Chat::remove_user_from_chat(request.into_inner().into(), &conn)
        .map(Json)
}

#[get("/belonging_to_user")]
fn get_chats_for_user(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<MinimalChatResponse>>> {
    Chat::get_chats_user_is_in(user.user_id, &conn)
        .map_vec::<MinimalChatResponse>()
        .map(Json)
}

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
