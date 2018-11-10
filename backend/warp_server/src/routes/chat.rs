use crate::{
    logging::{
        log_attach,
        HttpMethod,
    },
    state::{
        jwt::normal_user_filter,
        State,
    },
    util::{
        convert_and_json,
        convert_vector_and_json,
        json_body_filter,
    },
    uuid_integration::uuid_wrap_filter,
};
use db::chat::{
    Chat,
    ChatData,
    ChatUserAssociation,
    NewChat,
};
use error::Error;
use identifiers::{
    chat::ChatUuid,
    user::UserUuid,
};
use pool::PooledConn;
use warp::{
    filters::BoxedFilter,
    reply::Reply,
    Filter,
};
use wire::chat::{
    ChatResponse,
    ChatUserAssociationRequest,
    MinimalChatResponse,
    NewChatRequest,
};

pub fn chat_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Chat API");
    let api = create(s)
        .or(add_user_to_chat(s))
        .or(remove_user_from_chat(s))
        .or(get_owned_chats(s))
        .or(get_chat(s));

    warp::path("chat").and(api).with(warp::log("chat")).boxed()
}

pub fn create(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "chat/");

    warp::post2()
        .and(json_body_filter(12))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewChatRequest, user_uuid: UserUuid, conn: PooledConn| {
            let mut new_chat: NewChat = request.into();
            new_chat.leader_uuid = user_uuid.0;
            Chat::create_chat(new_chat, &conn)
                .map(convert_and_json::<Chat, MinimalChatResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn add_user_to_chat(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "chat/add_user");

    warp::put2()
        .and(warp::path("add_user"))
        .and(json_body_filter(12))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(
            |request: ChatUserAssociationRequest, user_uuid: UserUuid, conn: PooledConn| {
                if !Chat::is_user_in_chat(&request.chat_uuid, user_uuid, &conn).map_err(Error::simple_reject)? {
                    info!("User not in a chat tried to add a user to that chat.");
                    return Error::BadRequest.reject();
                }
                let association: ChatUserAssociation = request.into();
                Chat::add_user_to_chat(association, &conn)
                    .map(|_| warp::http::StatusCode::OK)
                    .map_err(Error::simple_reject)
            },
        )
        .boxed()
}

pub fn remove_user_from_chat(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "chat/remove_user");

    warp::put2()
        .and(warp::path("remove_user"))
        .and(json_body_filter(12))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(
            |request: ChatUserAssociationRequest, user_uuid: UserUuid, conn: PooledConn| {
                if !Chat::is_user_in_chat(&request.chat_uuid, user_uuid, &conn).map_err(Error::simple_reject)? {
                    info!("User not in a chat tried to remove a user from that chat.");
                    return Error::BadRequest.reject();
                }
                let association: ChatUserAssociation = request.into();
                Chat::remove_user_from_chat(association, &conn)
                    .map(|_| warp::http::StatusCode::OK)
                    .map_err(Error::simple_reject)
            },
        )
        .boxed()
}

pub fn get_owned_chats(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "chat/owned");

    warp::get2()
        .and(warp::path("owned"))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: UserUuid, conn: PooledConn| {
            Chat::get_chats_user_is_in(user_uuid, &conn)
                .map(convert_vector_and_json::<Chat, MinimalChatResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn get_chat(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "chat/");

    warp::get2()
        .and(uuid_wrap_filter())
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|chat_uuid: ChatUuid, _user: UserUuid, conn: PooledConn| {
            Chat::get_full_chat(chat_uuid, &conn)
                .map(convert_and_json::<ChatData, ChatResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}
