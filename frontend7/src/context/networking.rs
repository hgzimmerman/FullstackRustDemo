use super::Context;

use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::callback::Callback;


use requests_and_responses::user::*;
use requests_and_responses::thread::*;
use requests_and_responses::login::*;
use failure::Error;
use serde_json;
use serde::Serialize;

//use context::storage;

enum Auth {
    Required,
    NotRequired,
}


#[derive(Serialize)]
pub enum RequestWrapper {
    Login(LoginRequest),
    CreateUser(NewUserRequest),
    CreateThread(NewThreadRequest),
    GetThreads { forum_id: i32, page_index: usize },
    GetForums,
    GetForum { forum_id: i32 },
    GetThread { thread_id: i32 }
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
            GetThread { thread_id } => format!("{}/thread/{}", api_base, thread_id)
        }
    }
}


impl Context {
    pub fn make_request<W>(&mut self, request: RequestWrapper, callback: Callback<Response<W>>) -> Result<FetchTask, Error>
    where
        W: From<Result<String, Error>> + 'static,
    {

        let url = RequestWrapper::resolve_url(&request);

        use self::RequestWrapper::*;
        match request {
            Login(ref r) => {
                let request = self.prepare_post_request(
                    r,
                    url,
                    Auth::NotRequired,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            CreateUser(ref r) => {
                let request = self.prepare_post_request(
                    r,
                    url,
                    Auth::NotRequired,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            CreateThread(ref r) => {
                let request = self.prepare_post_request(
                    r,
                    url,
                    Auth::Required,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            GetThreads { .. } => {
                let request = self.prepare_get_request(
                    url,
                    Auth::NotRequired,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            GetForums => {
                let request = self.prepare_get_request(
                    url,
                    Auth::NotRequired,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            GetForum { .. } => {
                let request = self.prepare_get_request(
                    url,
                    Auth::NotRequired,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
            GetThread { .. } => {
                let request = self.prepare_get_request(
                    url,
                    Auth::NotRequired,
                )?;
                Ok(self.networking.fetch(request, callback))
            }
        }
    }

    fn prepare_post_request<T: Serialize>(&mut self, request_object: T, url: String, auth_requirement: Auth) -> Result<Request<String>, Error> {
        let body = serde_json::to_string(&request_object)
            .unwrap();
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
