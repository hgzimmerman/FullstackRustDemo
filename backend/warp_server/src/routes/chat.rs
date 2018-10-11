use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
use crate::db_integration::db_filter;
use db::Conn;
use uuid::Uuid;
//use db::RetrievableUuid;
use crate::convert_and_json;
use crate::convert_vector_and_json;
use crate::json_body_filter;
use crate::jwt::normal_user_filter;
use wire::chat::NewChatRequest;
use identifiers::user::UserUuid;
use wire::chat::MinimalChatResponse;
use db::Chat;
use db::chat::NewChat;
use db::CreatableUuid;
use wire::chat::ChatUserAssociationRequest;
use db::chat::ChatUserAssociation;
use crate::uuid_integration::uuid_filter;
use wire::chat::ChatResponse;
use db::chat::ChatData;
use identifiers::chat::ChatUuid;



pub fn chat_api() -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Chat API");
    let api = create()
        .or(add_user_to_chat())
        .or(remove_user_from_chat())
        .or(get_owned_chats())
        .or(get_chat())
        ;

    warp::path("chat")
        .and(api)
        .with(warp::log("chat"))
        .boxed()
}

pub fn create() -> BoxedFilter<(impl Reply,)> {
    warp::post2()
        .and(json_body_filter(12))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: NewChatRequest, user_uuid: UserUuid, conn: Conn | {
            let mut new_chat: NewChat = request.into();
            new_chat.leader_uuid = user_uuid.0;
            Chat::create(new_chat, &conn)
                .map(convert_and_json::<Chat,MinimalChatResponse>)
                .map_err(Error::convert_and_reject)

        })
        .boxed()
}

pub fn add_user_to_chat() -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(warp::path("add_user"))
        .and(json_body_filter(12))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: ChatUserAssociationRequest, user_uuid: UserUuid, conn: Conn | {
            if !Chat::is_user_in_chat(&request.chat_uuid, user_uuid, &conn).map_err(Error::convert_and_reject)? {
                info!("User not in a chat tried to add a user to that chat.");
                return Error::BadRequest.reject()
            }
            let association: ChatUserAssociation = request.into();
            Chat::add_user_to_chat(association, &conn)
                .map(|_|warp::http::StatusCode::OK)
                .map_err(Error::convert_and_reject)

        })
        .boxed()
}

pub fn remove_user_from_chat() -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(warp::path("remove_user"))
        .and(json_body_filter(12))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: ChatUserAssociationRequest, user_uuid: UserUuid, conn: Conn | {
            if !Chat::is_user_in_chat(&request.chat_uuid, user_uuid, &conn).map_err(Error::convert_and_reject)? {
                info!("User not in a chat tried to remove a user from that chat.");
                return Error::BadRequest.reject()
            }
            let association: ChatUserAssociation = request.into();
            Chat::remove_user_from_chat(association, &conn)
                .map(|_|warp::http::StatusCode::OK)
                .map_err(Error::convert_and_reject)

        })
        .boxed()
}


pub fn get_owned_chats() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path("owned"))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|user_uuid: UserUuid, conn: Conn|{
            Chat::get_chats_user_is_in(user_uuid, &conn)
                .map(convert_vector_and_json::<Chat,MinimalChatResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

pub fn get_chat() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(uuid_filter())
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|chat_uuid: Uuid, _user: UserUuid, conn: Conn|{
            let chat_uuid = ChatUuid(chat_uuid);
            Chat::get_full_chat(chat_uuid, &conn)
                .map(convert_and_json::<ChatData,ChatResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}