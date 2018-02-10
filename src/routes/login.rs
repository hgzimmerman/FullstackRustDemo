use rocket::Route;

use super::Routable;

use rocket_contrib::Json;
use user::User;
use db::Conn;
use rocket::State;

use auth;
use auth::LoginRequest;
use auth::LoginResult;
use auth::Secret;

// #[get("/admin", rank = 2)]
// fn login() -> Html<&'static str>{
//     Html(
//         "<form action=\"/api/login/admin\" method=\"POST\">
//         <input type=\"hidden\" name=\"username\" />
//         <input type=\"hidden\" name=\"password\" />
//         <input type=\"submit\" value=\"Login\" />
//     </form>"
//     )
// }

// #[post("/admin", data = "<form>")]
// fn login_post(form: Form<LoginStatus<DummyAuthenticator>>, cookies: Cookies) -> LoginRedirect{
//     // creates a response with either a cookie set (in case of a succesfull login)
//     // or not (in case of a failure). In both cases a "Location" header is send.
//     // the first parameter indicates the redirect URL when successful login,
//     // the second a URL for a failed login
//     form.into_inner().redirect("/api/login/admin", "/api/login/admin", cookies)
// }


#[post("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>, secret: State<Secret>, conn: Conn) -> LoginResult {
    auth::login(login_request.0, secret.clone().0, &conn)
}


pub fn login_routes() -> Vec<Route> {
    routes![login]
}


pub struct Login {}
impl Routable for Login {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| login_routes();
    const PATH: &'static str = "/login";
}