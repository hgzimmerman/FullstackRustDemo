use crate::jwt;
use crate::db_integration;
use db::Conn;
use warp;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use db::auth as auth_db;
use crate::error::Error;
use auth::Secret;
use wire::login::LoginRequest;
use auth::ServerJwt;
use crate::logging::log_attach;
use crate::logging::HttpMethod;

pub fn auth_api() -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching Auth API");
    warp::path("auth")
        .and(
            reauth()
                .or(login())
        )
        .with(warp::log("auth"))
        .boxed()
}


///// Logs the user in.
///// If successful, it generates a JWT which is used to verify other actions.
//#[post("/login", data = "<login_request>")]
//fn login(login_request: Json<LoginRequest>, secret: State<Secret>, conn: Conn) -> LoginResult {
//    auth_db::login(login_request.into_inner(), &secret, &conn)
//}
//
///// Given just a JWT from the header, verify the JWT,
///// and produce another JWT with an expiry time farther out in the future.
//#[get("/reauth")]
//fn reauth(jwt: ServerJwt, secret: State<Secret>) -> LoginResult {
//    auth_db::reauth(jwt, &secret)
//}


fn reauth() -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "auth/reauth");
    warp::get2()
        .and(warp::path("reauth"))
        .and(jwt::secret_filter())
        .and(jwt::jwt_filter())
        .and_then(|secret: Secret, jwt: ServerJwt| {
            auth_db::reauth(jwt, &secret)
                .map_err(|_| Error::NotAuthorized.simple_reject())
        })
        .boxed()
}

fn login() -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "auth/login");
    warp::post2()
        .and(warp::path("login"))
        .and(jwt::secret_filter())
        .and(db_integration::db_filter())
        .and(warp::body::json())
        .and_then(|secret: Secret, conn: Conn, login_request: LoginRequest| {
            auth_db::login(login_request, &secret, &conn)
                .map_err(|_| Error::NotAuthorized.simple_reject())
        })
        .boxed()
}