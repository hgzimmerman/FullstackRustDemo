use yew::services::fetch::FetchTask;
use Context;
use chrono::NaiveDateTime;
use chrono::Duration;
use stdweb::Value;
use stdweb::unstable::TryInto;
use failure::Error;
use yew::services::fetch::FetchService;
use yew::services::route::RouteService;
use yew::services::storage::{StorageService, Area};
use wire::user::Jwt;
use user;


//use yew::html::Env;
//use yew::html::Component;
use yew::callback::Callback;
use yew::format::Nothing;
use yew::services::fetch::Request;

use stdweb::web::Window;
use stdweb::web::window;
use stdweb::web::History;



#[derive(Default)]
pub struct JwtService {
    refresh_fetch_task: Option<FetchTask>,
}

//#[derive(Clone)]
//struct Activator {
//}
//
//impl Activator {
//    fn handle_message(&mut self, msg: JwtMsg) {
//        match msg {
//            JwtMsg::SetJwt(_) => {},
//            JwtMsg::RedirectToLogin => {}
//        }
//    }
//}
//
//enum JwtMsg {
//    SetJwt(String),
//    RedirectToLogin
//}

use yew::format::Json;
use yew::services::fetch::Response;

impl JwtService {
    fn refresh_jwt(&mut self, jwt_string: String, fetch_service: &mut FetchService, route_service: &mut RouteService) {
        // TODO, the route service needs to implement clone now.
        let rs = route_service.clone_without_listener();
        let closure = move |response: Response<Json<Result<String, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let jwt: String = data.unwrap();
                    let mut storage_service = StorageService::new(Area::Local);
                    storage_service.store("jwt", jwt)

                } else {
                    let mut storage_service = StorageService::new(Area::Local);
                    storage_service.remove("jwt");
                    // TODO this needs to take a string as well, or I could just import the routes.
                    let mut rs = rs.clone_without_listener();
                    rs.set_route_from_string("auth/login".into())
                }
            };
        let callback = Callback::from(closure);

        let url = String::from("api/auth/reauth");
        let request = Request::get(url.as_str())
                        .header("Content-Type", "application/json")
                        .header("Authorization", jwt_string.as_str())
                        .body(Nothing)
                        .unwrap();

        self.refresh_fetch_task = Some(fetch_service.fetch(request, callback));
    }
}

impl Context {
    /// Gets the JWT.
    /// If it exists, get the current time and see if it needs to refresh.
    /// Regardless of whether the jwt is refreshing or not, the current jwt string will be returned if it exists.
    /// The refresh will only occur if a JWT already exists.
    pub fn get_and_refresh_jwt(&mut self) -> Result<String, Error> {
        // You could not have a jwt stored
        let jwt_string: String = self.restore_jwt()?;
        // The stored jwt may be malformed
        let jwt: Jwt = user::extract_payload_from_jwt(jwt_string.clone())?;

        // Get current time from js
        let current_time_as_seconds: Value = js! {
            return new Date().getSeconds();
        };

        let current_time_as_seconds: i64 = current_time_as_seconds.try_into().expect("Couldn't convert local time into rust integer");
        let current_date = NaiveDateTime::from_timestamp(current_time_as_seconds, 0);

        // If current time > iat + 1 day, then refresh.
        if current_date > jwt.iat + Duration::days(1) {
            self.jwt_service.refresh_jwt(jwt_string.clone(), &mut self.networking, &mut self.routing )
        }

        // You get the old string regardless.
        // But because you are making this request anyway, the fact that the request
        // produced a momentary error in the view can be ignored as the user should be redirected by the callback
        // to the login screen.
        Ok(jwt_string)
    }
}