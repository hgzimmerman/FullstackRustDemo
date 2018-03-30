#![feature(try_from)]


#[macro_use]
extern crate yew;
extern crate requests_and_responses;
extern crate failure;
extern crate serde;
#[macro_use] extern crate serde_json;

extern crate chrono;

#[macro_use] extern crate stdweb;


use yew::prelude::*;
use yew::html::Scope;
use yew::services::console::ConsoleService;
// use counter::{Counter, Color};
// use barrier::Barrier;
mod datatypes;
use datatypes::minimal_thread::MinimalThread;
//mod header_component;
use header_component::Header;

mod components;
use components::*;

mod services;

use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use services::route_service::RouteService;

use auth::AuthPage;
use components::forum::Forum;

pub struct Context {
    // console: ConsoleService,
    networking: FetchService,
    routing: RouteService
}

/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
// impl counter::Printer for Context {
//     fn print(&mut self, data: &str) {
//         self.console.log(data);
//     }
// }
//
#[derive(Clone, PartialEq, Debug)]
pub enum PageView {
    ForumView,
    ArticleView,
    AuthView(AuthPage),
    BucketView,
}

impl Default for PageView {
    fn default() -> Self {
        PageView::ForumView
    }
}

pub trait Routable<T> {
    fn route(context: &mut Context) -> T;

}

struct Model {
    page: PageView,
//    jwt: Option<String> // Its dumb to store it here, but for now, this is where the jwt will live. Instead it should use the localstorage api.
}


enum Msg {
    Login{ jwt: String },
    Logout,
    NoOp,
    Navigate(PageView)
}

impl Routable<PageView> for Model {
    fn route(context: &mut Context) -> PageView {
        match context.routing.get_route().as_ref() {
//            "auth" => PageView::AuthView(AuthPage::Undetermined),
            "forum" => PageView::ForumView,
            "article" => PageView::ArticleView,
            "bucket" => PageView::BucketView,
            _ => PageView::BucketView // default to bucket questions
        }
    }
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        Model {
            page: Model::route(context),
//            jwt: None
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {

        self.page = Model::route(context);
        println!("updating model");


        match msg {
            Msg::Login {jwt} => {
                // Set the jwt
//                self.jwt = Some(jwt);
                true
            }
            Msg::Logout => {
                // Invalidate the JWT
//                self.jwt = None;
                // Navigate elsewhere
                self.page = PageView::AuthView(AuthPage::Login);
                true
            }
            Msg::NoOp => {
                true
            }
            Msg::Navigate(page) => {
//                context.routing.set_route("");
                println!("MainNav: navigating to {:?}", page);
                self.page = page;
                true
            }
        }
    }
}

use components::auth::Auth;

use components::header_component::*;


impl Renderable<Context, Model> for Model {

    fn view(&self) -> Html<Context, Self> {


        let page = || {
            match self.page {
                PageView::AuthView(ref auth_page) => {
                    html! {
                        <>
                            <Auth: child=auth_page, callback=|_| Msg::Navigate(PageView::ForumView), />
                        </>
                    }
                }
                PageView::ForumView => {
                    html! {
                        <>
                            <Forum: child=None, />
                        </>
                    }
                }
                _ => {
                    html! {
                        <>
                            {"Main routing Not implemented"}
                        </>
                    }
                }
            }
        };

        let header_links = vec![
            HeaderLink {
                name: "Login".into(),
                link: PageView::AuthView(AuthPage::Login)
            },
            HeaderLink {
                name: "Forum".into(),
                link: PageView::ForumView
            }
        ];
        use link::Link;
        html! {
            <>
                <Header: links=header_links, callback=|pv| Msg::Navigate(pv), />
                {page()}
            </>
        }
    }
}



fn main() {
    yew::initialize();
    stdweb::initialize(); // I need this in order to use my route service
    let context = Context {
        networking: FetchService::new(),
        routing: RouteService::new()
    };
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
