//#![feature(try_from)]


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


use yew::services::route::*;

//mod routing;
//use routing::*;


//use yew::context::fetch::{FetchService, FetchTask, Request, Response};

use auth::AuthRoute;
use components::forum::forum_list::ForumListRoute;
use components::forum::forum_list::ForumList;
use components::auth::Auth;
use components::header_component::*;

/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
// impl counter::Printer for Context {
//     fn print(&mut self, data: &str) {
//         self.console.log(data);
//     }
// }
//
#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Forums(ForumListRoute),
//    ArticleView,
    Auth(AuthRoute),
//    BucketView,
    PageNotFound
}


impl <'a> From<&'a RouteInfo> for Route {
    fn from(route_info: &RouteInfo) -> Self {
        println!("Converting from url");
        if let Some(first_segment) = route_info.get_segment_at_index(0) {
            println!("matching: {}", first_segment);
            match first_segment {
                "forum" => return Route::Forums(ForumListRoute::from(route_info)),
                "auth" => return Route::Auth(AuthRoute::from(route_info)),
                _ => return Route::PageNotFound
            }
        }
        Route::PageNotFound
    }
}

impl Into<RouteInfo> for Route {
    fn into(self) -> RouteInfo {
        match self {
            Route::Forums(forum_list_route)=> RouteInfo::parse("/forum").unwrap() + forum_list_route.into(),
            Route::Auth(auth_route) => RouteInfo::parse("/auth").unwrap() + auth_route.into(),
            Route::PageNotFound => RouteInfo::parse("/pagenotfound").unwrap()
        }
    }
}
impl From<RouteResult> for Msg {
    fn from( result: RouteResult) -> Self {
        match result {
            Ok(route_info) => {
               Msg::Navigate(Route::from(&route_info))
            }
            Err(e) => {
                eprintln!("Couldn't route: {:?}", e);
                Msg::Navigate(Route::PageNotFound)
            }
        }
    }

}

impl Default for Route {
    fn default() -> Self {
        Route::Forums(ForumListRoute::default())
    }
}


struct Model {
    route: Route,
}


enum Msg {
    Navigate(Route),
}




impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {

//        let cb = context.send_back(|path: String| {
//            println!("Callback path changed {}", path);
//            let path_components = path.split('/').collect::<Vec<&str>>().into_iter().map(str::to_string).collect::<Vec<String>>();
//            Msg::Navigate(Route::route(path_components))
//        });
//
//        context.routing.register_callback(cb);

        let callback = context.send_back(|route_result: RouteResult| {
            Msg::from(route_result)
        });
        context.routing.register_router(callback);


        let route: Route = (&context.routing.get_current_route_info()).into();
        context.routing.replace_url(route.clone()); // sets the url to be dependent on what the route_info was resolved to

        Model {
            route
        }
    }

    fn update(&mut self, msg: Msg, _context: &mut Env<Context, Self>) -> ShouldRender {

        println!("updating model");
        match msg {
            Msg::Navigate(route) => {
                println!("MainNav: navigating to {:?}", route);
                self.route = route;
                true
            }
        }
    }
}




impl Renderable<Context, Model> for Model {

    fn view(&self) -> Html<Context, Self> {
        println!("Rendering main");

        let page = |route: &Route| {
            match route {
                Route::Auth(ref auth_page) => {
                    html! {
                        <>
                            <Auth: child=auth_page, />
                        </>
                    }
                }
                Route::Forums(ref forum_list_route) => {
                    println!("ForumView chosen to render by main with parameters {:?}", forum_list_route);
                    html! {
                        <>
                            <ForumList: route=forum_list_route, />
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
//            HeaderLink {
//                name: "Forum".into(),
//                link: Route::Forums(ForumListRoute::List)
//            },
//            HeaderLink {
//                name: "Login".into(),
//                link: Route::Auth(AuthRoute::Login)
//            },
        ];
        html! {
            <div class="main-container", >
                <Header: links=header_links, callback=|pv| Msg::Navigate(pv), />
                <div class="main-content", >
                    {page(&self.route)}
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
