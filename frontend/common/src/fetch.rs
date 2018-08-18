//use yew::prelude::*;
use yew::prelude::worker::*;
use yew::services::FetchService;
use failure::Error;
use yew::services::fetch::FetchTask;
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;
use yew::format::Nothing;
use yew::format::Text;
use yew::services::fetch::Response;
use serde::Deserialize;
use serde_json;
use yew_router::router_agent::{Router, RouterRequest};
use yew_router::Route;
use wire::user::Jwt;
use super::user;
use chrono::{Duration};

use yew::services::fetch::Request;
use yew::services::storage::{StorageService, Area};

use yew::callback::Callback;

use wire::user::BEARER;
#[derive(Debug)]
pub enum Auth {
    Required,
    NotRequired,
}

/// Not all are included, but isn't intended to be exhaustive
pub enum HttpMethod {
    Get,
    Post(String),
    Put(String),
    Delete
}

/// Anything that implements this trait can be used to send requests.
pub trait FetchRequest: Serialize + DeserializeOwned {
    fn resolve_path(&self) -> String;
    fn resolve_auth(&self) -> Auth;
    fn resolve_body_and_method(&self) -> HttpMethod;

    fn resolve_url(&self) -> String {
        let api_base: &str = if cfg!(feature = "development") {
            "http://localhost:8001/api"
        } else {
            "http://www.weekendatjo.es/api"
        };

        let path: String = self.resolve_path();
        let path = path.trim_left_matches('/');
        format!("{}/{}", api_base, path)
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FetchError {
    /// Could not deserialize the response into the type defined as W.
    DecodeError(String),
    /// If the response came back as a 401.
    Unauthorized,
    /// Authentication wasn't present when the request was made.
    AuthAbsent,
    /// Not found
    ResourceNotFound,
    /// Bad request
    BadRequest,
    /// Forbidden: 403 you are authenticated, but not authorized to access this resource.
    Forbidden,
    /// Unhandled error.
    Misc
}

#[derive(Serialize, Deserialize)]
pub enum FetchResponse<T> {
    /// Indicates that the request worked as intended.
    Success(T),
    /// Something went wrong with the request.
    Error(FetchError),
    /// Fetch Connection has started
    Started
}

impl <T> Clone for FetchResponse<T> where T: Clone {
    fn clone(&self) -> Self {
        use self::FetchResponse::*;
        match self {
            Success(t) => Success(t.clone()),
            a => a.clone()
        }
    }
}

impl<T> FetchResponse<T> {
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> FetchResponse<U> {
        match self {
            FetchResponse::Success(t) => FetchResponse::Success(f(t)),
            FetchResponse::Error(e) => FetchResponse::Error(e),
            FetchResponse::Started => FetchResponse::Started
        }
    }
}

impl <W> Transferable for FetchResponse<W> where W: Serialize + for <'de> Deserialize<'de> {}



pub enum Msg<W> {
    Data(HandlerId, FetchResponse<W>),
    NoOp
}

// ======== Use of the agent is discouraged. Just use the service instead. ==========
//
/// An agent that facilitates sending network requests as well as managing authentication and route redirection.
struct FetchAgent<T, W>
    where T: FetchRequest + Transferable + 'static,
        W: for<'de> Deserialize<'de> + Serialize + 'static
{
    fetch_service: FetchService,
    storage_service: StorageService,
    link: AgentLink<FetchAgent<T, W>>,
    /// Used to hold on to fetch tasks that are used for reauthentication.
    fetch_task_collection: Vec<FetchTask>,
    router: Router,
    phantom: PhantomData<T>,
    phantom_w: PhantomData<W>
}

impl <T, W> Agent for FetchAgent<T, W>
    where T: FetchRequest + Transferable + 'static,
          W: for<'de> Deserialize<'de> + Serialize + 'static,
          Self: 'static
{
    type Reach = Context;
    type Message = Msg<W>;
    type Input = T;
    type Output = FetchResponse<W>;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.send_back(|_: Route| Msg::NoOp);
        let router = Router::new(callback);

        FetchAgent {
            fetch_service: FetchService::new(),
            storage_service: StorageService::new(Area::Local),
            link,
            fetch_task_collection: Vec::new(),
            router,
            phantom: PhantomData,
            phantom_w: PhantomData
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Data(who, fetch_response) => match fetch_response {
                FetchResponse::Error(error) => {
                    match error {
                        FetchError::Unauthorized
                        | FetchError::AuthAbsent => self.router.send(RouterRequest::ChangeRoute(Route::parse("/auth/login"))),
                        _ => {}
                    }
                    self.link.response(who, FetchResponse::Error(error))
                },
                fetch_response => self.link.response(who, fetch_response),
            }
            Msg::NoOp => {}
        }
    }

    fn handle(&mut self, request: Self::Input, who: HandlerId) {
        // Define the callback that will fire when the request comes back.
        let whom = who;
        let callback = self.link.send_back(move |response: Response<Text>| {
            let (meta, data) = response.into_parts();
            let message = match meta.status.into() {

                401 => {
                    FetchResponse::Error(FetchError::Unauthorized)
                }
                200 ... 299 => {
                    let data: String = data.unwrap();
                    let data: FetchResponse<W> = serde_json::from_str(&data)
                        .map(FetchResponse::Success)
                        .unwrap_or_else(|_| FetchResponse::Error(FetchError::DecodeError(data)));
                    data
                }
                _ => {
                    // not handled
                    FetchResponse::Error(FetchError::Misc)
                }
            };
            Msg::Data(whom, message)
        });

        // Get the relevant information from the request.
        let url: String = request.resolve_url();
        let auth_requirement: Auth = request.resolve_auth();
        let body_and_method: HttpMethod = request.resolve_body_and_method();




        let mut request_builder = Request::builder();
        match body_and_method {
            HttpMethod::Get => {
                request_builder.method("GET")
            }
            HttpMethod::Post(_) => {
                request_builder.method("POST")
            }
            HttpMethod::Put(_) => {
                request_builder.method("PUT")
            }
            HttpMethod::Delete => {
                request_builder.method("DELETE")
            }
        };
        request_builder.uri(url.as_str());
        request_builder.header("Content-Type", "application/json");

        info!("Sending request to: '{}', with auth: {:?}", url, auth_requirement);

        // If the auth is required _and_ the user isn't logged in or their session is expired,
        // redirect the user to the login screen.
        if let Some(token) = user::get_token_if_valid(&mut self.storage_service) {
            refresh_jwt_if_needed(&mut self.fetch_service, &token, &mut self.fetch_task_collection);
            request_builder.header("Authorization", format!("{} {}", BEARER, token).as_str());
        } else if let Auth::Required = auth_requirement {
            self.update(Msg::Data(who, FetchResponse::Error(FetchError::AuthAbsent)));
            return; // don't continue
        }

        let body: Option<String> = match body_and_method {
            HttpMethod::Get => {
                None
            }
            HttpMethod::Post(body) => {
                Some(body)
            }
            HttpMethod::Put(body) => {
                Some(body)
            }
            HttpMethod::Delete => {
                None
            }
        };

        // Make the request
        let task = if let Some(body) = body {
            let body = Ok(body);
            let request: Request<Result<String, Error>> = request_builder.body(body).unwrap();
            self.fetch_service.fetch(request, callback)
        } else {
            let request: Request<Nothing> = request_builder.body(Nothing).unwrap();
            self.fetch_service.fetch(request, callback)
        };
        // Hold on to the task so it isn't dropped and canceled.
        self.fetch_task_collection.push(task);
        self.link.response(who, FetchResponse::Started);

        // Remove tasks that aren't holding anything.
        use yew::services::Task;
        self.fetch_task_collection.retain(|ref x| x.is_active());

    }
}

#[derive(Serialize, Deserialize)]
/// Simple reauth request
struct Reauth;

impl FetchRequest for Reauth {
    fn resolve_path(&self) -> String {
        "/auth/reauth".into()
    }

    fn resolve_auth(&self) -> Auth {
        Auth::Required
    }

    fn resolve_body_and_method(&self) -> HttpMethod {
        HttpMethod::Get
    }
}

fn refresh_jwt(jwt_string: &str, fetch_service: &mut FetchService, fetch_task_collection: &mut Vec<FetchTask>) {
    info!("JWT is being refreshed, this will extend the length of login session's validity.");

    let closure = move |response: Response<Result<String, Error>>| {
        let (meta, data) = response.into_parts();
        println!("META: {:?}, {:?}", meta, data);
        if meta.status.is_success() {
            info!("New JWT retrieved.");
            let jwt = data;
            if jwt.is_err() {
                error!("New JWT encountered an error")
            }
            let mut storage_service = StorageService::new(Area::Local);
            info!("Storing JWT");
            storage_service.store("JWT", jwt);
        } else {
            let mut storage_service = StorageService::new(Area::Local);
            storage_service.remove("JWT");
            let callback = Callback::from(|_| ());// NOOP
            let mut router = Router::new(callback);
            let route = Route::parse("/auth/login");
            router.send(RouterRequest::ChangeRoute(route));
        }
    };
    let callback = Callback::from(closure);

    let url = Reauth.resolve_url();
    let request = Request::get(url.as_str())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("{} {}", BEARER, jwt_string).as_str())
        .body(Nothing)
        .unwrap();

    // This is never culled, as the FetchTasks themselves are so small,
    // and these requests made so rarely, that a webpage left up for years would not suffer
    // anything approaching a slowdown due to vector expansion.
    fetch_task_collection.push(fetch_service.fetch(request, callback));
}

/// If a specific amount of time has elapsed since the jwt has been issued, then refresh the jwt.
fn refresh_jwt_if_needed(fetch_service: &mut FetchService, jwt_string: &str, fetch_task_collection: &mut Vec<FetchTask>)  {
        // The stored jwt may be malformed 
        // By sending a default jwt, it will fail the reauth, logging the user out. This path should never be needed, but would fail in a safe manner.
        let jwt: Jwt = user::extract_payload_from_jwt(jwt_string).unwrap_or_default();


        let current_date = user::get_now();
//        self.log(&format!("current: {:?}, iat: {:?}", current_date, jwt.iat) );
        // If current time > iat + 1 day, then refresh.
        if current_date > jwt.iat + Duration::days(1) {
//            self.log("Refreshing JWT");
            refresh_jwt(jwt_string, fetch_service, fetch_task_collection);
        }
}




pub struct Networking {
    /// Used for fetching
    fetch_service: FetchService,
    /// Gets the JWT
    storage_service: StorageService,
    /// Used to hold on to fetch tasks.
    fetch_task_collection: Vec<FetchTask>,
    router: RouterSenderBase<()>,
}
use std::fmt::Debug;
impl Debug for Networking {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Networking {{ num_held_tasks: {} }}", self.fetch_task_collection.len())
    }
}

use yew::html::ComponentLink;
use yew::html::Renderable;
use yew::html::Component;
use yew_router::router_agent::RouterSenderBase;


pub fn to_body(r: &impl Serialize) -> String {
    serde_json::to_string(r).unwrap()
}
impl Networking {
    pub fn new<T>(link: &ComponentLink<T>) -> Self
        where
        T: Component + Renderable<T>,
        T::Message: Default
    {
        let callback = link.send_back(|_| T::Message::default());
        let router = RouterSenderBase::<()>::new(callback);
        Networking {
            fetch_service: FetchService::new(),
            storage_service: StorageService::new(Area::Local),
            fetch_task_collection: Vec::new(),
            router
        }
    }

    pub fn fetch<T, U, V>(&mut self, request: &T, response_mapper: fn(FetchResponse<U>) -> V::Message, link: &ComponentLink<V>)
        where
            T: FetchRequest,
            U: for <'de> Deserialize<'de> + 'static,
            V: Component + Renderable<V>,
            V::Message: Default
    {
        let router_cb = link.send_back(|_| V::Message::default());
        let handle_response_closure = move |response: Response<Text>| {
            debug!("Response received");
            let (meta, data) = response.into_parts();
            let message: FetchResponse<U> = match meta.status.into() {
                401 => {
                    // This will_not send messages back to the component to to which this fetch struct was created from.
                    let mut router = RouterSenderBase::<()>::new(router_cb.clone());
                    router.send(RouterRequest::ChangeRoute(Route::parse("/auth/login")));
                    warn!("Response: Not Authorized. Authentication not present or has expired");
                    FetchResponse::Error(FetchError::Unauthorized)
                }
                404 => {
                    FetchResponse::Error(FetchError::ResourceNotFound)
                }
                403 => {
                    FetchResponse::Error(FetchError::Forbidden)
                }
                400 => {
                    FetchResponse::Error(FetchError::BadRequest)
                }
                200...299 => {
                    info!("Response: 2xx response");
                    let data: String = data.unwrap();
                    let data: FetchResponse<U> = serde_json::from_str(&data)
                        .map(FetchResponse::Success)
                        .unwrap_or_else(|_| FetchResponse::Error(FetchError::DecodeError(data)));
                    data
                }
                _ => {
                    // not handled
                    FetchResponse::Error(FetchError::Misc)
                }
            };

            if cfg!(feature = "development") {
                match &message {
                    FetchResponse::Success(_) => info!("Response: Successful"),
                    FetchResponse::Error(error) => warn!("Response: Error: {:?}", error),
                    _ => {}
                };
            }
            message
        };

        let closure = move |response: Response<Text>| {
            let fetch_response = handle_response_closure(response);
            response_mapper(fetch_response)
        };
        let callback = link.send_back(closure);


        if let Some(task) = make_request(self, request, callback ) {
            // Hold on to the task so it isn't dropped and canceled.
            self.fetch_task_collection.push(task);
            // Remove tasks that aren't holding anything.
            use yew::services::Task;
            self.fetch_task_collection.retain(|ref x| x.is_active());
        }

        // Pass the "Request Started" message back to the calling component.
        let started_callback = link.send_back(response_mapper);
        started_callback.emit(FetchResponse::Started)
    }


    pub fn fetch_string<T, V>(&mut self, request: &T, response_mapper: fn(FetchResponse<String>) -> V::Message, link: &ComponentLink<V>)
        where
            T: FetchRequest,
            V: Component + Renderable<V>,
            V::Message: Default
    {
        let router_cb = link.send_back(|_| V::Message::default()); // do nothing
        let handle_response_closure = move |response: Response<Text>| {
            debug!("Response received");
            let (meta, data) = response.into_parts();
            let message: FetchResponse<String> = match meta.status.into() {
                401 => {
                    // This will_not send messages back to the component to to which this fetch struct was created from.
                    let mut router = RouterSenderBase::<()>::new(router_cb.clone());
                    router.send(RouterRequest::ChangeRoute(Route::parse("/auth/login")));
                    warn!("Response: Not Authorized. Authentication not present or has expired, redirecting to login");
                    FetchResponse::Error(FetchError::Unauthorized)
                }
                404 => {
                    FetchResponse::Error(FetchError::ResourceNotFound)
                }
                403 => {
                    FetchResponse::Error(FetchError::Forbidden)
                }
                400 => {
                    FetchResponse::Error(FetchError::BadRequest)
                }
                200...299 => {
                    info!("Response: 2xx response");
                    let data: String = data.unwrap();
                    FetchResponse::Success(data)
                }
                _ => {
                    // not handled
                    FetchResponse::Error(FetchError::Misc)
                }
            };

            if cfg!(feature = "development") {
                match &message {
                    FetchResponse::Success(_) => info!("Response: Successful"),
                    FetchResponse::Error(error) => warn!("Response: Error: {:?}", error),
                    _ => {}
                };
            }
            message
        };

        let closure = move |response: Response<Text>| {
            let fetch_response = handle_response_closure(response);
            response_mapper(fetch_response)
        };
        let callback = link.send_back(closure);


        if let Some(task) = make_request(self, request, callback ) {
            // Hold on to the task so it isn't dropped and canceled.
            self.fetch_task_collection.push(task);
            // Remove tasks that aren't holding anything.
            use yew::services::Task;
            self.fetch_task_collection.retain(|ref x| x.is_active());
        }

        // Pass the "Request Started" message back to the calling component.
        let started_callback = link.send_back(response_mapper);
        started_callback.emit(FetchResponse::Started)
    }
}

fn make_request<T: FetchRequest>(fetch_struct: &mut Networking, request: &T, callback: Callback<Response<Text>>) -> Option<FetchTask> {
        // Get the relevant information from the request.
    let url: String = request.resolve_url();
    let auth_requirement: Auth = request.resolve_auth();
    let body_and_method: HttpMethod = request.resolve_body_and_method();


    let mut request_builder = Request::builder();
    match body_and_method {
        HttpMethod::Get => {
            request_builder.method("GET")
        }
        HttpMethod::Post(_) => {
            request_builder.method("POST")
        }
        HttpMethod::Put(_) => {
            request_builder.method("PUT")
        }
        HttpMethod::Delete => {
            request_builder.method("DELETE")
        }
    };
    request_builder.uri(url.as_str());
    request_builder.header("Content-Type", "application/json");


    // If the auth is required _and_ the user isn't logged in or their session is expired,
    // redirect the user to the login screen.
    if let Some(token) = user::get_token_if_valid(&mut fetch_struct.storage_service) {
        refresh_jwt_if_needed(&mut fetch_struct.fetch_service, &token, &mut fetch_struct.fetch_task_collection);
        request_builder.header("Authorization", format!("{} {}", BEARER, token).as_str());
    } else if let Auth::Required = auth_requirement {
        fetch_struct.router.send(RouterRequest::ChangeRoute(Route::parse("/auth/login")));
        return None; // don't continue
    }

    let body: Option<String> = match body_and_method {
        HttpMethod::Get => {
            None
        }
        HttpMethod::Post(body) => {
            Some(body)
        }
        HttpMethod::Put(body) => {
            Some(body)
        }
        HttpMethod::Delete => {
            None
        }
    };

    info!("Sending request to: '{}', with auth: {:?}", url, auth_requirement);

    // Make the request
    let task = if let Some(body) = body {
        let body = Ok(body);
        let request: Request<Result<String, Error>> = request_builder.body(body).unwrap();
        fetch_struct.fetch_service.fetch(request, callback)
    } else {
        let request: Request<Nothing> = request_builder.body(Nothing).unwrap();
        fetch_struct.fetch_service.fetch(request, callback)
    };
    Some(task)
}

