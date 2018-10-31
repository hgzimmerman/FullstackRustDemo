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

use error::warp_support::customize_error;
use crate::state::State;

pub const API_STRING: &str = "api";


/// Combine the API with the static file handler.
/// Any missed GETs that doesn't start with '/api' will redirect to the index.html.
/// Also support CORS, as that should be applied to the whole server.
pub fn routes(s: &State) -> BoxedFilter<(impl warp::Reply,)> {
    api(&s)
        .or(static_files_handler())
        .recover(customize_error) // Top level error correction
        .or(cors()) // For some reason, this needs to come after the recover() section.
        .boxed()
}

fn api(s: &State) -> BoxedFilter<(impl warp::Reply,)> {

    let api = auth_api(s)
        .or(user_api(s))
        .or(article_api(s))
        .or(answer_api(s))
        .or(bucket_api(s))
        .or(chat_api(s))
        .or(forum_api(s))
        .or(message_api(s))
        .or(post_api(s))
        .or(question_api(s))
        .or(thread_api(s))
    ;

    warn!("Attaching Main API");
    warp::path(API_STRING)
        .and(api)
        .with(warp::log(API_STRING))
        .boxed()
}




/// sort of a fake cors implementation.
fn cors() -> BoxedFilter<(impl warp::Reply,)> {
    // TODO replace this once a blessed implementation is released by warp
    warp::options()
        .and(warp::header::<String>("origin"))
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
        })
        .boxed()
}


#[cfg(test)]
mod tests {
    use super::*;
//    use crate::error::Error;

    #[test]
    fn routes_redirect_to_index() {
        assert!(
            warp::test::request()
                .path("/yeet")
                .filter(&routes(&State::default()))
                .is_ok()
        )
    }

    #[test]
    fn routes_invalid_api_path_still_404s() {
        let resp = warp::test::request()
            .path("/api/yeet") // Matches nothing in the API space
            .reply(&routes(&State::default()));

        let status = resp.status();
        assert_eq!(status, 404);

    }
}
