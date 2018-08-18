use yew::prelude::*;
use yew_router::prelude::*;
//use Context;
use util::button::*;

//use yew::services::fetch::{Response};
//use failure::Error;
use wire::login::*;
//use context::networking::*;
//use super::AuthRoute;
use util::uploadable::Uploadable;
use yew_router::components::RouterButton;
use yew_router::router_agent::RouterSenderBase;
use requests::AuthRequest;
use common::fetch::FetchResponse;
use common::fetch::Networking;

use common::user::{LoginAgent, LoginRequest as LoginStoreRequest, LoginResponse};
//use routes::Route;
//use routes::forum::ForumRoute;
//
//use routes::routing::Router;

pub enum Msg {
    UpdatePassword(String),
    UpdateUserName(String),
    Submit,
    LoginRequestSuccess(String),
    LoginRequestStarted,
    NoOp,
    LoginRequestError,
    HandleLoginStoreResponse(LoginResponse)
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

#[derive(Debug, Default, Clone)]
pub struct LoginData {
    user_name: String,
    password: String
}


pub struct Login {
    login_data: Uploadable<LoginData>,
    networking: Networking,
    link: ComponentLink<Login>,
    router: RouterSenderBase<()>,
//    storage_service: StorageService
    login_agent: Box<Bridge<LoginAgent>>,
}

impl Routable for Login {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if let Some(seg_2) = route.path_segments.get(1) {
            if seg_2 == "login" {
                return Some(())
            }
        }
        return None
    }
    fn will_try_to_route(route: &Route) -> bool {
        if let Some(seg_1) = route.path_segments.get(0) {
            seg_1.as_str() == "auth"
        } else {
            false
        }
    }
}


impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cb = link.send_back(|_| Msg::default());

        let login_agent = LoginAgent::bridge(link.send_back(|response| Msg::HandleLoginStoreResponse(response)));
        Login {
            login_data: Uploadable::default(),
//            on_login_cb: props.on_login_cb,
            networking: Networking::new(&link),
            router: RouterSenderBase::<()>::new(cb),
            link,
            login_agent
//            storage_service: StorageService::new(Area::Local)
        }
    }


    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Submit => {
                fn response_mapper(fetch_response: FetchResponse<String>) -> Msg { // TODO figure out what response type.
                    match fetch_response {
                        FetchResponse::Started => Msg::LoginRequestStarted,
                        FetchResponse::Success(jwt_string) => Msg::LoginRequestSuccess(jwt_string),
                        FetchResponse::Error(_) => Msg::LoginRequestError
                    }
                };


                let login_data = self.login_data.cloned_inner();

                let login_request: LoginRequest = LoginRequest {
                    user_name: login_data.user_name,
                    password: login_data.password,
                };

                self.networking.fetch_string(
                    &AuthRequest::Login(login_request),
                    response_mapper,
                    &self.link
                );
                true
            }

            Msg::UpdatePassword(p) => {
                self.login_data.as_mut().password = p;
                true
            }
            Msg::UpdateUserName(u) => {
                self.login_data.as_mut().user_name = u;
                true
            }
            Msg::LoginRequestSuccess(jwt_string) => {
//                context.store_jwt(jwt.clone()); // store/upsert the local JWT.
//                use common;
//                common::user::store_jwt(&mut self.storage_service, jwt);

//                context.log(&format!("Logged in. JWT received with payload: {:?}", ::context::user::extract_payload_from_jwt(jwt)));
                self.login_agent.send(LoginStoreRequest::Login{jwt_string});
//                self.router.send(RouterRequest::ChangeRoute(Route::parse("forums/")));
                self.router.send(RouterRequest::ChangeRoute(route!("forums")));

//                context.routing.set_route(Route::Forums(ForumRoute::ForumList).to_route().to_string());

                true
            }
            Msg::LoginRequestError => {
                self.login_data.set_failed("Login Failed, try another user name combo");
                true
            }
            Msg::LoginRequestStarted => {
                self.login_data.set_uploading();
                true
            }
            Msg::HandleLoginStoreResponse(response) => {
                match response {
                    LoginResponse::LoggedIn(_) => {
                        self.router.send(RouterRequest::ChangeRoute(route!("forums/")));
                    }
                    LoginResponse::LoggedOut => {
                    }
                }
                true
            }
            Msg::NoOp => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//        self.on_login_cb = props.on_login_cb;
        true
    }
}

impl Renderable<Login> for Login {
    fn view(&self) -> Html<Self> {
        fn login_view(login_data: &LoginData) -> Html<Login> {
            html! {
                <div class=("login-card", "flexbox-vert"),>
                    <div class="flexbox-child-grow",>
                        <h3>
                            {"Login"}
                        </h3>
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="User Name",
                            value=&login_data.user_name,
                            oninput=|e| Msg::UpdateUserName(e.value),
                            onkeypress=|e| {
                                if e.key() == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                        />
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="Password",
                            value=&login_data.password,
                            oninput=|e| Msg::UpdatePassword(e.value),
                            onkeypress=|e| {
                                if e.key() == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                            type="password",
                        />
                    </div>

                    <div class=("flexbox-horiz"),>
                        <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
                        <RouterButton: text="Create Account", route=route!("auth/create"), />
                    </div>
                </div>
            }
        }
        html! {
            <div class=("full-height","scrollable", "flexbox"),>
                <div class="flexbox-center-item",>
                    {self.login_data.default_view(login_view)}
                </div>
            </div>
        }

    }
}
