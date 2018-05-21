use super::Context;

use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::callback::Callback;


use wire::user::*;
use wire::thread::*;
use wire::login::*;
use wire::post::*;
use wire::bucket::*;
use wire::answer::*;
use wire::question::*;

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
    /*Auth*/
    Login(LoginRequest),
    CreateUser(NewUserRequest),
    /*Forum*/
    CreateThread(NewThreadRequest),
    GetThreads { forum_id: i32, page_index: usize },
    GetForums,
    GetForum { forum_id: i32 },
    GetThread { thread_id: i32 },
    CreatePostResponse(NewPostRequest),
    UpdatePost(EditPostRequest),
    /*Bucket Questions*/
    GetPublicBuckets,
    GetBucketsForUser,
    GetBucket{bucket_id: i32},
    CreateBucket(NewBucketRequest),
    GetRandomQuestion { bucket_id: i32 },
    GetQuestions { bucket_id: i32},
    AnswerQuestion(NewAnswerRequest),
    CreateQuestion(NewQuestionRequest),
    DeleteQuestion{question_id: i32},
    PutQuestionBackInBucket{question_id: i32},
    SetBucketPublicStatus{bucket_id: i32, is_public: bool},
    ApproveUserForBucket {bucket_id: i32, user_id: i32},
    RemoveUserFromBucket {bucket_id: i32, user_id: i32}
}

impl RequestWrapper {
    pub fn resolve_url(request: &RequestWrapper) -> String {
        let api_base = "http://localhost:8001/api"; // TODO Make this build-time configurable

        use self::RequestWrapper::*;
        let path: String = match *request {
            Login(_) => "auth/login".into(),
            CreateUser(_) => "user/".into(),
            CreateThread(_) => "thread/create".into(),
            GetThreads {
                forum_id,
                page_index,
            } => format!("thread/get/{}/{}", forum_id, page_index),
            GetForums => "forum/forums".into(),
            GetForum { forum_id } => format!("forum/{}", forum_id),
            GetThread { thread_id } => format!("thread/{}", thread_id),
            CreatePostResponse(_) => "post/create".into(),
            UpdatePost(_) => "post/edit".into(),
            GetPublicBuckets => "buckets/public".into(),
            GetBucketsForUser => "buckets/approved".into(),
            GetBucket{bucket_id} => format!("buckets/{}", bucket_id),
            CreateBucket(_) => "buckets/create".into(),
            GetRandomQuestion { bucket_id } => format!("question/random_question/{}", bucket_id),
            GetQuestions { bucket_id } => format!("question/questions_in_bucket/{}", bucket_id),
            AnswerQuestion(_) => "answer/create".into(),
            CreateQuestion(_) => "question/create".into(),
            DeleteQuestion {question_id} => format!("question/{}", question_id),
            PutQuestionBackInBucket {question_id} => format!("question/{}/into_bucket", question_id),
            SetBucketPublicStatus {bucket_id, is_public} => format!("buckets/{}/publicity?is_public={}", bucket_id, is_public),
            ApproveUserForBucket {bucket_id, user_id} => format!("buckets/{}/approval?user_id={}",bucket_id, user_id),
            RemoveUserFromBucket {bucket_id, user_id} => format!("buckets/{}?user_id={}",bucket_id, user_id),
        };

        format!("{}/{}", api_base, path)
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
            GetPublicBuckets => Required,
            GetBucketsForUser => Required,
            GetBucket{..} => Required,
            CreateBucket(_) => Required,
            GetRandomQuestion {..} => NotRequired,
            GetQuestions {..} => NotRequired,
            AnswerQuestion(_) => Required,
            CreateQuestion(_) => Required,
            DeleteQuestion {..} => Required,
            PutQuestionBackInBucket {..} => Required,
            SetBucketPublicStatus {..} => Required,
            ApproveUserForBucket {..} => Required,
            RemoveUserFromBucket {..} => Required,

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
            GetPublicBuckets => Get,
            GetBucketsForUser => Get,
            GetBucket {..} => Get,
            CreateBucket(r) => Post(to_body(r)),
            GetRandomQuestion {..} => Get,
            GetQuestions {..} => Get,
            AnswerQuestion(r) => Post(to_body(r)),
            CreateQuestion(r) => Post(to_body(r)),
            DeleteQuestion {..} => Delete,
            PutQuestionBackInBucket {..} => Put("".to_string()), // no body
            SetBucketPublicStatus {..} => Put("".to_string()),
            ApproveUserForBucket {..} => Put("".to_string()),
            RemoveUserFromBucket {..} => Delete
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
                let request = self.prepare_delete_request(
                    url,
                    auth_requirement,
                )?;
                Ok(self.networking.fetch(request, callback))
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
    fn prepare_delete_request(&mut self, url: String, auth_requirement: Auth) -> Result<Request<Nothing>, Error> {
        match self.restore_jwt() {
            Ok(jwt_string) => {
                // TODO: possibly check if the jwt is outdated here before sending
                Ok(
                    Request::delete(url.as_str())
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
                            Request::delete(url.as_str())
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
