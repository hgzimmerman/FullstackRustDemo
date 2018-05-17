use super::Context;

use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::callback::Callback;


use wire::user::*;
use wire::thread::*;
use wire::login::*;
use wire::post::*;
use wire::bucket::*;
use failure::Error;
use serde_json;
use serde::Serialize;

//use context::storage;

pub trait FtWrapper{
    fn set_ft(&mut self, fetch_task: FetchTask);
}

enum Auth {
    Required,
    NotRequired,
}

/// Not all are included, but isn't intended to be exhaustive
enum HttpMethod {
    Get,
    Post(String),
    Put(String),
    #[allow(dead_code)]
    Delete
}


#[derive(Serialize)]
pub enum RequestWrapper {
    Login(LoginRequest),
    CreateUser(NewUserRequest),
    CreateThread(NewThreadRequest),
    GetThreads { forum_id: i32, page_index: usize },
    GetForums,
    GetForum { forum_id: i32 },
    GetThread { thread_id: i32 },
    CreatePostResponse(NewPostRequest),
    UpdatePost(EditPostRequest),
    GetBuckets,
    GetBucket{bucket_id: i32},
    CreateBucket(NewBucketRequest)
}

impl RequestWrapper {
    pub fn resolve_url(request: &RequestWrapper) -> String {
        let api_base = "http://localhost:8001/api"; // TODO Make this build-time configurable

        use self::RequestWrapper::*;
        match *request {
            Login(_) => format!("{}/auth/login", api_base),
            CreateUser(_) => format!("{}/user/", api_base),
            CreateThread(_) => format!("{}/thread/create", api_base),
            GetThreads {
                forum_id,
                page_index,
            } => format!("{}/thread/get/{}/{}", api_base, forum_id, page_index),
            GetForums => format!("{}/forum/forums", api_base),
            GetForum { forum_id } => format!("{}/forum/{}", api_base, forum_id),
            GetThread { thread_id } => format!("{}/thread/{}", api_base, thread_id),
            CreatePostResponse(_) => format!("{}/post/create", api_base),
            UpdatePost(_) => format!("{}/post/edit", api_base),
            GetBuckets => format!("{}/buckets/", api_base),
            GetBucket{bucket_id} => format!("{}/buckets/{}", api_base, bucket_id),
            CreateBucket(_) => format!("{}/buckets/create", api_base),
        }
    }

    fn resolve_auth(&self) -> Auth {

        use self::RequestWrapper::*;
        use self::Auth::*;
        match self {
            Login(_) => NotRequired,
            CreateUser(_) => NotRequired,
            CreateThread(_) => Required,
            GetThreads {..} =>  NotRequired,
            GetForums => NotRequired,
            GetForum {..} => NotRequired,
            GetThread {..} => NotRequired,
            CreatePostResponse(_) => Required,
            UpdatePost(_) => Required,
            GetBuckets => Required,
            GetBucket{..} => Required,
            CreateBucket(_) => Required
        }
    }

    fn resolve_body_and_method(&self) -> HttpMethod {

        fn to_body(r: &impl Serialize) -> String {
            serde_json::to_string(r).unwrap()
        }

        use self::HttpMethod::*;
        use self::RequestWrapper::*;
        match self {
            Login(r) => Post(to_body(r)),
            CreateUser(r) => Post(to_body(r)),
            CreateThread(r) => Post(to_body(r)),
            GetThreads {..} => Get,
            GetForums => Get,
            GetForum {..} => Get,
            GetThread {..} => Get,
            CreatePostResponse(r) => Post(to_body(r)),
            UpdatePost(r) => Put(to_body(r)),
            GetBuckets => Get,
            GetBucket {..} => Get,
            CreateBucket(r) => Post(to_body(r))
        }
    }
}


impl Context {

    /// Make a request, that if the conditions to send a JWT aren't met, the user will be logged out.
    /// This will also set the object that encapsulates the fetch task to be in its loading state.
    pub fn make_logoutable_request<W, FTW>(&mut self, ft_wrapper: &mut FTW, request: RequestWrapper, callback: Callback<Response<W>>)
    where
        W: From<Result<String, Error>> + 'static,
        FTW: FtWrapper + Sized
    {
        match self.make_request(request, callback) {
            Ok(ft) => ft_wrapper.set_ft(ft),
            Err(_) => {
                use Route;
                use components::auth::AuthRoute;
                self.routing.set_route(Route::Auth(AuthRoute::Login));
            }
        }
    }

    /// The error in the result here should only occur if the JWT is outdated, or not present,
    /// in which case, the caller should redirect to the login screen.
    pub fn make_request<W>(&mut self, request: RequestWrapper, callback: Callback<Response<W>>) -> Result<FetchTask, Error>
    where
        W: From<Result<String, Error>> + 'static,
    {

        let url: String = RequestWrapper::resolve_url(&request);
        let auth_requirement: Auth = request.resolve_auth();
        let body_and_method: HttpMethod = request.resolve_body_and_method();

        match body_and_method {
            HttpMethod::Get => {
                let request = self.prepare_get_request(
                    url,
                    auth_requirement,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            HttpMethod::Post(body) => {
                let request = self.prepare_post_request(
                    body,
                    url,
                    auth_requirement
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            HttpMethod::Put(body) => {
                let request = self.prepare_put_request(
                    body,
                    url,
                    auth_requirement
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            HttpMethod::Delete => {
                unimplemented!()
            }
        }

    }


    fn prepare_put_request(&mut self, body: String, url: String, auth_requirement: Auth) -> Result<Request<String>, Error> {
        match self.restore_jwt() {
            Ok(jwt_string) => {
                // TODO: possibly check if the jwt is outdated here before sending
                Ok(
                    Request::put(url.as_str())
                        .header("Content-Type", "application/json")
                        .header("Authorization", jwt_string.as_str())
                        .body(body)
                        .unwrap(),
                )
            }
            Err(e) => {
                match auth_requirement {
                    Auth::Required => {
                        eprintln!("JWT was not found for a request that requires it: '{}'", url);
                        Err(e)
                    }
                    // If the auth wasn't required in the first place
                    Auth::NotRequired => {
                        Ok(
                            Request::put(url.as_str())
                                .header("Content-Type", "application/json")
                                .body(body)
                                .unwrap(),
                        )
                    }
                }

            }
        }
    }

    fn prepare_post_request(&mut self, body: String, url: String, auth_requirement: Auth) -> Result<Request<String>, Error> {
/*        let body = serde_json::to_string(&request_object)
            .unwrap();*/
        match self.restore_jwt() {
            Ok(jwt_string) => {
                // TODO: possibly check if the jwt is outdated here before sending
                Ok(
                    Request::post(url.as_str())
                        .header("Content-Type", "application/json")
                        .header("Authorization", jwt_string.as_str())
                        .body(body)
                        .unwrap(),
                )
            }
            Err(e) => {
                match auth_requirement {
                    Auth::Required => {
                        eprintln!("JWT was not found for a request that requires it: '{}'", url);
                        Err(e)
                    }
                    // If the auth wasn't required in the first place
                    Auth::NotRequired => {
                        Ok(
                            Request::post(url.as_str())
                                .header("Content-Type", "application/json")
                                .body(body)
                                .unwrap(),
                        )
                    }
                }

            }
        }
    }

    fn prepare_get_request(&mut self, url: String, auth_requirement: Auth) -> Result<Request<Nothing>, Error> {
        match self.restore_jwt() {
            Ok(jwt_string) => {
                // TODO: possibly check if the jwt is outdated here before sending
                Ok(
                    Request::get(url.as_str())
                        .header("Content-Type", "application/json")
                        .header("Authorization", jwt_string.as_str())
                        .body(Nothing)
                        .unwrap(),
                )
            }
            Err(e) => {
                match auth_requirement {
                    Auth::Required => {
                        eprintln!("JWT was not found for a request that requires it: '{}'", url);
                        Err(e)
                    }
                    // If the auth wasn't required in the first place
                    Auth::NotRequired => {
                        Ok(
                            Request::get(url.as_str())
                                .header("Content-Type", "application/json")
                                .body(Nothing)
                                .unwrap(),
                        )
                    }
                }

            }
        }
    }
}
