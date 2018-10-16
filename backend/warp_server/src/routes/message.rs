use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
use crate::db_integration::db_filter;
use db::Conn;
use uuid::Uuid;
//use db::RetrievableUuid;
use crate::util::convert_and_json;
use crate::util::convert_vector_and_json;
//use crate::uuid_integration::uuid_filter;
use crate::util::json_body_filter;
use identifiers::user::UserUuid;
use crate::jwt::normal_user_filter;
use db::Message;
use db::Chat;
use identifiers::chat::ChatUuid;
use wire::message::MessageResponse;
use db::message::MessageData;
use wire::message::NewMessageRequest;
use db::message::NewMessage;
use crate::logging::log_attach;
use crate::logging::HttpMethod;
use crate::util::query_uuid;

pub fn message_api() -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Message API");
    let api = get_messages_for_chat()
        .or(send_message())
        ;

    warp::path("message")
        .and(api)
        .with(warp::log("message"))
        .boxed()
}


fn get_messages_for_chat() -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "message/<index=i32>?chat_uuid=<uuid>");

    warp::get2()
        .and(warp::path::param())
        .and(query_uuid("chat_uuid")) // TODO Is this the query??
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|index: i32, chat_uuid: Uuid, user_uuid: UserUuid, conn: Conn|{
            let chat_uuid = ChatUuid(chat_uuid);
            if !Chat::is_user_in_chat(&chat_uuid, user_uuid, &conn).map_err(Error::convert_and_reject)? {
                return Error::BadRequest.reject()
            }

            Message::get_messages_for_chat(chat_uuid, index, 25, &conn)
                .map(convert_vector_and_json::<MessageData, MessageResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn send_message() -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "message/send");

    warp::post2()
        .and(warp::path::path("send"))
        .and(json_body_filter(20))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: NewMessageRequest, user_uuid: UserUuid, conn: Conn|{
            if !Chat::is_user_in_chat(&request.chat_uuid, user_uuid, &conn).map_err(Error::convert_and_reject)? {
                return Error::BadRequest.reject()
            }
            if request.author_uuid != user_uuid {
                return Error::BadRequest.reject()
            }

            let new_message: NewMessage = request.into();
            Message::create_message(new_message, &conn)
                  .map(convert_and_json::<MessageData, MessageResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}