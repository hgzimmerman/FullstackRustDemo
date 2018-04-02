#![feature(try_from)]


#[macro_use]
extern crate yew;
extern crate requests_and_responses;
#[macro_use] extern crate failure_derive;
extern crate failure;
extern crate serde;
//#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

#[macro_use] extern crate stdweb;

extern crate pulldown_cmark;

extern crate base64;


use yew::prelude::*;
use yew::html::Scope;
//use yew::context::console::ConsoleService;
// use counter::{Counter, Color};
// use barrier::Barrier;
mod datatypes;
//mod header_component;
use header_component::Header;

mod components;
use components::*;

mod context;
pub use context::Context;

//use yew::context::fetch::{FetchService, FetchTask, Request, Response};

use auth::AuthPage;
use components::forum::forum_list::ForumList;


/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
// impl counter::Printer for Context {
//     fn print(&mut self, data: &str) {
//         self.console.log(data);
//     }
// }
//
#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    ForumView,
    ArticleView,
    AuthView(AuthPage),
    BucketView,
}

impl Default for Route {
    fn default() -> Self {
        Route::ForumView
    }
}

pub enum Router<T: Routable> {
    Route(T),
    Path(Vec<String>)
}

impl<T: Routable> Router<T> {
    fn resolve_route(self) -> T {
        match self {
            Router::Route(route) => route,
            Router::Path(path_components) => T::route(path_components)
        }
    }
}

pub trait Routable {
    fn route(path_components: Vec<String>) -> Self;
}

struct Model {
    page: Route,
}


enum Msg {
    Navigate(Route),
}

impl Routable for Route {
    fn route(path_components: Vec<String>) -> Route {

        // The route is given in the form "/path/path/path"
        // The string at index 0 is "" because of the first "/", so get at index 1 here
        if let Some(first) = path_components.get(1) {
            println!("First path component is: {}", first);
            match first.as_str() {
                "auth" => Route::AuthView(AuthPage::Login),
                "forum" => Route::ForumView,
                "article" => Route::ArticleView,
                "bucket" => Route::BucketView,
                _ => Route::BucketView // default to bucket questions
            }
        } else {
            Route::default()
        }
    }
}


impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        let cb = context.send_back(|path: String| {
            println!("Callback path changed {}", path);
            let path_components = path.split('/').collect::<Vec<&str>>().into_iter().map(str::to_string).collect::<Vec<String>>();
            Msg::Navigate(Route::route(path_components))
        });

        context.routing.register_callback(cb);


        Model {
            page: Route::default(),
        }
    }

    fn update(&mut self, msg: Msg, _context: &mut Env<Context, Self>) -> ShouldRender {

        println!("updating model");
        match msg {
            Msg::Navigate(route) => {
//                context.routing.set_route("");
                println!("MainNav: navigating to {:?}", route);
                self.page = route;
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
                Route::AuthView(ref auth_page) => {
                    html! {
                        <>
                            <Auth: child=auth_page, callback=|_| Msg::Navigate(Route::ForumView), />
                        </>
                    }
                }
                Route::ForumView => {
                    html! {
                        <>
                            <ForumList: child=None, />
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
                name: "Forum".into(),
                link: Route::ForumView
            },
            HeaderLink {
                name: "Login".into(),
                link: Route::AuthView(AuthPage::Login)
            },
        ];
        html! {
            <div class="main-container", >
                <Header: links=header_links, callback=|pv| Msg::Navigate(pv), />
                <div class="main-content", >
                    {page()}
                </div>
            <div/>
        }
    }
}



fn main() {
    yew::initialize();
    stdweb::initialize(); // I need this in order to use my route service
    let context = Context::new();
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
