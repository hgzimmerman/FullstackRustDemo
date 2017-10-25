use auth::userpass::UserPass;
use rocket::request::Form;
use rocket::response::content::Html;
use rocket::http::Cookies;
use auth::status::{LoginStatus,LoginRedirect};
use rocket::Route;
use auth::authenticator::Authenticator;
use std::collections::HashMap;
use auth::dummy::DummyAuthenticator;

use super::user::StoredUser;
use super::Routable;

//#[derive(Debug)]
//pub struct JoeAuthenticator {
//    storedUser: StoredUser
//}
//
//impl Authenticator for JoeAuthenticator {
//    type User = StoredUser;
//
//    fn user(&self) -> StoredUser {
//        self.storedUser
//    }
//
//    fn check_credentials(username: String, password: String) -> Result<Self, Self>{
//        if username
//    }
//}




#[get("/admin")]
fn admin(info: UserPass<String>) -> String{
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

pub fn login_routes() -> Vec<Route> {
    routes![admin, login, login_post]
}


pub struct Login {}
impl Routable for Login {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![admin, login, login_post];
    const PATH: &'static str = "/login";
}