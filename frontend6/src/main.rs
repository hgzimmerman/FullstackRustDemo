#[macro_use]
extern crate yew;
extern crate requests_and_responses;
extern crate failure;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate stdweb;


use yew::prelude::*;
use yew::html::Scope;
use yew::services::console::ConsoleService;
// use counter::{Counter, Color};
// use barrier::Barrier;
mod datatypes;
use datatypes::minimal_thread::MinimalThread;
// mod threadCardComponent;
use threadCardComponent::ThreadCard;
//mod header_component;
use header_component::Header;

mod components;
use components::*;

mod services;

use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use services::route_service::RouteService;

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
pub enum PageView {
    ForumView,
    ArticleView,
    AuthView,
    BucketView,
}

pub trait Routable<T> {
    fn route(context: &mut Context) -> T;

}

struct Model {
    page: PageView,
    jwt: Option<String> // Its dumb to store it here, but for now, this is where the jwt will live. Instead it should use the localstorage api.
}


enum Msg {
    Login{ jwt: String },
    Logout,
    NoOp,
    Navigate(PageView)
}

impl Routable<PageView> for Model {
    fn route(context: &mut Context) -> PageView {
        match context.routing.route.as_ref() {
            "auth" => PageView::AuthView,
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
            jwt: None
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {

        self.page = Model::route(context);
        println!("updating model");


        match msg {
            Msg::Login {jwt} => {
                // Set the jwt
                self.jwt = Some(jwt);
                true
            }
            Msg::Logout => {
                // Invalidate the JWT
                self.jwt = None;
                // Navigate elsewhere
                self.page = PageView::AuthView;
                true
            }
            Msg::NoOp => {
                true
            }
            Msg::Navigate(page) => {
                match page {
                    _ => {
                        println!("Setting page")
                    }
                };
                self.page = page;
                true
            }
        }
    }
}

use components::auth::Auth;

impl Renderable<Context, Model> for Model {

    fn view(&self) -> Html<Context, Self> {


        let page = || {
            match self.page {
                PageView::AuthView => {
                    html! {
                        <>
                            <Auth: callback=|_| Msg::Navigate(PageView::ForumView), />
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

        use link::Link;
        html! {
            <>
                <div class="header",>
                    { "WeekendAtJoe dot com" }

                    <Link: name="login", callback=|_| Msg::Navigate(PageView::AuthView), />
                    <Link: name="Threads", callback=|_| Msg::Navigate(PageView::ForumView), />
                </div>
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
