use rocket::Route;

use super::Routable;

use rocket_contrib::Json;
use db::Conn;
use rocket::State;

use auth;
use wire::login::LoginRequest;
use auth::LoginResult;
use auth::Secret;


/// Logs the user in.
/// If successful, it generates a JWT which is used to verify other actions.
#[post("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>, secret: State<Secret>, conn: Conn) -> LoginResult {
    auth::login(login_request.0, secret.clone().0, &conn)
}

/// Acts as a namespace for auth related methods
pub struct Auth {}
impl Routable for Auth {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![login];
    const PATH: &'static str = "/auth";
}
