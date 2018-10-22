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
use pool::PooledConn;
use crate::state::State;

pub fn auth_api(s: &State) -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching Auth API");
    warp::path("auth")
        .and(
            reauth(s)
                .or(login(s))
        )
        .with(warp::log("auth"))
        .boxed()
}


fn reauth(s: &State) -> BoxedFilter<(impl Reply,)> {
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

fn login(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "auth/login");
    warp::post2()
        .and(warp::path("login"))
        .and(jwt::secret_filter())
        .and( s.db.clone())
        .and(warp::body::json())
        .and_then(|secret: Secret, conn: PooledConn, login_request: LoginRequest| {
            auth_db::login(login_request, &secret, &conn)
                .map_err(|_| Error::NotAuthorized.simple_reject())
        })
        .boxed()
}