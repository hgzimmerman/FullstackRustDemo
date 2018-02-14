use rocket::Route;

use super::Routable;

use rocket_contrib::Json;
use db::Conn;
use rocket::State;

use auth;
use auth::LoginRequest;
use auth::LoginResult;
use auth::Secret;


#[post("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>, secret: State<Secret>, conn: Conn) -> LoginResult {
    auth::login(login_request.0, secret.clone().0, &conn)
}

/// Acts as a namespace for auth related methods
pub struct Auth {}
impl Routable for Auth {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![login];
    const PATH: &'static str = "/auth";
}