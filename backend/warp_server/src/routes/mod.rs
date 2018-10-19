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
mod thread;
mod static_file;

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
use self::thread::thread_api;

pub use self::static_file::static_files_handler;


use warp;
use warp::Filter;

use crate::error::customize_error;
//use warp::reply::Reply;

pub const API_STRING: &str = "api";

pub fn api() -> BoxedFilter<(impl warp::Reply,)> {


    // sort of a fake cors implementation.
    // TODO replace this once a blessed implementation is released by warp
    let cors = warp::options()
        .and(warp::header("origin"))
        .map(|origin: String| {
            let with_header = warp::reply::with_header(
                warp::reply(),
                "access-control-allow-origin",
                origin
            );
            warp::reply::with_header(
                with_header,
                "vary",
                "origin"
            )
        });

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
        .or(thread_api())
    ;

    warn!("Attaching Main API");
    warp::path(API_STRING)
        .and(
            api
            .or(cors)
        )
        .recover(customize_error)
        .with(warp::log(API_STRING))
        .boxed()
}