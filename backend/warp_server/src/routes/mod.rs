use warp::filters::BoxedFilter;

mod user;
mod auth;

use self::user::user_api;
use self::auth::auth_api;

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


    warn!("Attaching Main API");
    warp::path("api")
//        .and(cors)
        .and(
            auth_api()
                .or(user_api())
//            user_api()
        )
        .recover(customize_error)
        .with(warp::log("api"))
        .boxed()
}