#[macro_use]
extern crate yew;
extern crate failure;
#[macro_use]
extern crate yew_router;
use yew_router::prelude::*;
//extern crate context;
extern crate common;
//extern crate wire;
extern crate util;
//extern crate routes;

use yew::prelude::*;
use yew_router::components::RouterLink;

use util::link::Link;
use yew_router::router_agent::RouterSenderBase;
//use yew_router::route::RouteBase;
use common::user::{LoginAgent, LoginRequest, LoginResponse};

//use util::link::Link;
//use context::Context;

//use routes::Route;
//use routes::auth::AuthRoute;
//use routes::forum::ForumRoute;
//use routes::bucket::BucketRoute;
//
//use routes::routing::Router;


pub struct Header {
    is_logged_in: bool,
    //    storage_service: StorageService,
    login_agent: Box<Bridge<LoginAgent>>,
    router: RouterSenderBase<()>,
}

pub enum Msg {
    InitiateLogout,
    HandleLoginResponse(LoginResponse),
    NoOp,
}



impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cb = link.send_back(|_| Msg::NoOp);


        let login_agent = LoginAgent::bridge(link.send_back(|response| {
            Msg::HandleLoginResponse(response)
        }));
        login_agent.send(LoginRequest::Query);
        Header {
            is_logged_in: false,
            login_agent,
            router: RouterSenderBase::<()>::new(cb),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            InitiateLogout => {
                self.login_agent.send(
                    LoginRequest::Logout,
                );
                self.is_logged_in = false;
                self.router.send(
                    RouterRequest::ChangeRoute(
                        Route::parse("auth/login"),
                    ),
                );
                true
            }
            HandleLoginResponse(response) => {
                match response {
                    LoginResponse::LoggedIn(_) => self.is_logged_in = true,
                    LoginResponse::LoggedOut => self.is_logged_in = false,
                }
                true
            }
            NoOp => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Header> for Header {
    fn view(&self) -> Html<Self> {

        let log_in_out = if self.is_logged_in {
            html! {
                <Link<()>: name="Logout", callback=|_| Msg::InitiateLogout, classes="nav-link", />
            }
        } else {
            html! {
                <RouterLink: text="Login", route=route!("auth/login"), />
            }
        };

        let bucket_questions = if self.is_logged_in {
            html! {
                <RouterLink: text="Bucket Questions", route=route!("bucket"), />
            }
        } else {
            util::wrappers::empty_vdom_node()
        };

        html! {
            <div class="header",>
                <div class="nav-title",>
                    { "WeekendAtJoe's ALPHA" }
                </div>
                <div class="nav-links",>
                    // Spans are necessary to keep the ordering preserved under different states.
                    <span>
                        <RouterLink: text="Forums", route=route!("forum"), />
                    </span>
                    <span>
                        {bucket_questions}
                    </span>
                    <span>
                        {log_in_out}
                    </span>
                </div>
            </div>
        }
    }
}
