use rocket::Route;

use super::Routable;

use pool::Conn;
use rocket::State;
use rocket_contrib::Json;

//use auth;
use auth_lib::{
    Secret,
    ServerJwt,
};
use db::auth as auth_db;
use error::LoginResult;
use wire::login::LoginRequest;

/// Logs the user in.
/// If successful, it generates a JWT which is used to verify other actions.
#[post("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>, secret: State<Secret>, conn: Conn) -> LoginResult {
    auth_db::login(login_request.into_inner(), &secret, &conn)
}

/// Given just a JWT from the header, verify the JWT,
/// and produce another JWT with an expiry time farther out in the future.
#[get("/reauth")]
fn reauth(jwt: ServerJwt, secret: State<Secret>) -> LoginResult {
    auth_db::reauth(jwt, &secret)
}

/// Acts as a namespace for auth related methods
pub struct Auth {}
impl Routable for Auth {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![login, reauth];
    const PATH: &'static str = "/auth";
}
