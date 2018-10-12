use warp::filters::BoxedFilter;

mod user;
mod auth;
mod article;
mod answer;
mod bucket;
mod chat;
mod forum;
mod message;
mod post;
mod question;

use self::user::user_api;
use self::auth::auth_api;
use self::article::article_api;
use self::answer::answer_api;
use self::bucket::bucket_api;
use self::chat::chat_api;
use self::forum::forum_api;
use self::message::message_api;
use self::post::post_api;
use self::question::question_api;

use warp;
use warp::Filter;

use crate::error::customize_error;

pub fn api() -> BoxedFilter<(impl warp::Reply,)> {

//    use warp::reply::Response;
//    let cors = warp::any()
//        .and(warp::header("origin").map(|origin: String| {
//            Response::builder()
//                .header("access-control-allow-origin", origin)
//                .header("vary", "origin")
//        }))
//        .or(warp::any().map(|| Response::builder()))
//        .unify();

    let api = auth_api()
        .or(user_api())
        .or(article_api())
        .or(answer_api())
        .or(bucket_api())
        .or(chat_api())
        .or(forum_api())
        .or(message_api())
        .or(post_api())
        .or(question_api())
    ;

    warn!("Attaching Main API");
    warp::path("api")
//        .and(cors)
        .and(api)
        .recover(customize_error)
        .with(warp::log("api"))
        .boxed()
}