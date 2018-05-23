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

use networking::RequestWrapper;

use yew::services::fetch::Response;
use yew::callback::Callback;
use yew::format::Nothing;
use yew::services::fetch::Request;


#[derive(Default)]
pub struct JwtService {
    refresh_fetch_task: Option<FetchTask>,
}


// TODO use a constant for this value when const functions finally arrive.
//const REAUTH_DELAY: Duration = Duration::days(1);

impl JwtService {


    /// This whole method is is a next-gen level of jankyness,
    /// but it is what is required in order to be able to divorce the network request from the typical
    /// component lifetime binding.
    fn refresh_jwt(&mut self, jwt_string: String, fetch_service: &mut FetchService, route_service: &mut RouteService) {

        // This gets a mostly complete copy of the route service.
        // This particular service won't have a listener that listens for JS events related to changes in the URL bar.
        // But it does have a copy of the callback that ties into the Root component's route handler,
        // so setting the route here will cause the app to update.
        let rs = route_service.clone_without_listener();

        let closure = move |response: Response<Result<String, Error>>| {
                let (meta, data) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let jwt = data.expect("Expected JWT response to be a string");
                    let mut storage_service = StorageService::new(Area::Local);
                    storage_service.store("JWT", jwt);
                } else {
                    let mut storage_service = StorageService::new(Area::Local);
                    storage_service.remove("JWT");
                    let mut rs = rs.clone_without_listener();
                    rs.set_route_from_string("/auth/login".into())
                }
            };
        let callback = Callback::from(closure);

        let url = RequestWrapper::Reauth.resolve_url();
        let request = Request::get(url.as_str())
                        .header("Content-Type", "application/json")
                        .header("Authorization", jwt_string.as_str())
                        .body(Nothing)
                        .unwrap();

        // This will overwrite existing fetch tasks, so that only the most recent one will actually affect the app.
        // Because there is no protection against it here, it is quite possible to send multiple reauth requests.
        self.refresh_fetch_task = Some(fetch_service.fetch(request, callback));
    }
}

impl Context {

    /// Gets the JWT string.
    /// If it exists, get the current time and see if it needs to refresh.
    /// Regardless of whether the jwt is refreshing or not, the current jwt string will be returned if it exists.
    /// The refresh will only occur if a JWT already exists.
    pub fn get_and_refresh_jwt(&mut self) -> Result<String, Error> {
        // You could not have a jwt stored
        let jwt_string: String = self.restore_jwt()?;
        // The stored jwt may be malformed
        let jwt: Jwt = user::extract_payload_from_jwt(jwt_string.clone())?;

        // Get current unix timestamp from js
        let current_time_as_seconds: Value = js! {
            var d = new Date();
            return Math.floor(d.getTime() / 1000);
        };

        let current_time_as_seconds: i64 = current_time_as_seconds.try_into().expect("Couldn't convert local timestamp int into Rust i64");
        let current_date = NaiveDateTime::from_timestamp(current_time_as_seconds, 0);


//        self.log(&format!("current: {:?}, iat: {:?}", current_date, jwt.iat) );
        // If current time > iat + 1 day, then refresh.
        if current_date > jwt.iat + Duration::days(1) {
            self.log("Refreshing JWT");
            self.jwt_service.refresh_jwt(jwt_string.clone(), &mut self.networking, &mut self.routing );
        }

        // Because you get the old jwt string regardless, your intended request can go through without waiting first for the new JWT to arrive.
        // No slowdown should be encountered, although it does introduce some jank when actually kicking the user out.
        // A request authenticated by a super old JWT would fail before the app kicks the user out due to the reauth attempt.
        // This would cause a couple of frames to show the failed-to-load request.
        // This is deemed to be fine.
        Ok(jwt_string)
    }
}