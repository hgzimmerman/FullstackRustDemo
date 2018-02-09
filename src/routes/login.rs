use auth::userpass::UserPass;
use rocket::request::Form;
use rocket::response::content::Html;
use rocket::http::Cookies;
use auth::status::{LoginStatus,LoginRedirect};
use rocket::Route;
use auth::authenticator::Authenticator;
use std::collections::HashMap;
use auth::dummy::DummyAuthenticator;

use super::user::User;
use super::Routable;

use frank_jwt::{Algorithm, encode, decode};
use frank_jwt;
use chrono::{NaiveDateTime, Utc};
use rocket_contrib::Json;





static format_string: &'static str = "%Y-%m-%d %H:%M:%S";


fn generate_jwt(user_name: String, token_key; String, token_expire_date: NaiveDateTime, secret: String) -> String  {
    let header = json!({});

    let expired_date: String = user.token_expire_date.format(format_string).to_string();
    let payload = json!({
        "user_name": user_name,
        "token_key": token_key,
        "token_expire_date": expired_date
    });
    use std::ops::Deref;
    encode(header, &secret, &payload, Algorithm::ES256).unwrap()
}


#[get("/admin")]
fn admin(info: UserPass<String>) -> String {
    info!("{:?}", info.user);
    // we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
    format!("Restricted administration area, user logged in: {}", info.user)
}


#[get("/admin", rank = 2)]
fn login() -> Html<&'static str>{
    Html(
        "<form action=\"/api/login/admin\" method=\"POST\">
        <input type=\"hidden\" name=\"username\" />
        <input type=\"hidden\" name=\"password\" />
        <input type=\"submit\" value=\"Login\" />
    </form>"
    )
}

#[post("/admin", data = "<form>")]
fn login_post(form: Form<LoginStatus<DummyAuthenticator>>, cookies: Cookies) -> LoginRedirect{
    // creates a response with either a cookie set (in case of a succesfull login)
    // or not (in case of a failure). In both cases a "Location" header is send.
    // the first parameter indicates the redirect URL when successful login,
    // the second a URL for a failed login
    form.into_inner().redirect("/api/login/admin", "/api/login/admin", cookies)
}

#[post("/admin", data = "<form>")]
fn login(user_name: String, password: String) -> String {
    // get user

    // refresh token/ generate token

    // update entry with new values

    // return the token

}

pub fn login_routes() -> Vec<Route> {
    routes![admin, login, login_post]
}


pub struct Login {}
impl Routable for Login {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![admin, login, login_post];
    const PATH: &'static str = "/login";
}