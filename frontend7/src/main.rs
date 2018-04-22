//#![feature(try_from)]


#[macro_use]
extern crate yew;
extern crate requests_and_responses;
#[macro_use]
extern crate failure_derive;
extern crate failure;
extern crate serde;
//#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

//#[macro_use]
extern crate stdweb;

extern crate pulldown_cmark;

extern crate base64;

use yew::prelude::*;
mod datatypes;
use header_component::Header;

mod components;
use components::*;

mod context;
pub use context::Context;


use yew::services::route::*;


use components::auth::AuthRoute;
use components::forum::ForumRoute;
use components::forum::forum_list::ForumList;
use components::auth::Auth;
use components::header_component::*;


#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Forums(ForumRoute),
    //    ArticleView,
    Auth(AuthRoute),
    //    BucketView,
    PageNotFound,
}


impl Router for Route {
    fn to_route(&self) -> RouteInfo {
        match *self {
            Route::Forums(ref forum_list_route) => RouteInfo::parse("/forum").unwrap() + forum_list_route.to_route(),
            Route::Auth(ref auth_route) => RouteInfo::parse("/auth").unwrap() + auth_route.to_route(),
            Route::PageNotFound => {
                RouteInfo::parse("/pagenotfound")
                    .unwrap()
            }
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        Some(Self::from_route_main(route))
    }
}

impl MainRouter for Route {
    fn from_route_main(route: &mut RouteInfo) -> Self {
        if let Some(RouteSection::Node { segment }) = route.next() {
            match segment.as_str() {
                "forum" => {
                    if let Some(child) = ForumRoute::from_route(route) {
                        Route::Forums(child)
                    } else {
                        Route::PageNotFound
                    }
                }
                "auth" => {
                    if let Some(child) = AuthRoute::from_route(route) {
                        Route::Auth(child)
                    } else {
                        Route::PageNotFound
                    }
                }
                _ => Route::PageNotFound,
            }
        } else {
            Route::PageNotFound
        }
    }
}


impl From<RouteResult> for Msg {
    fn from(result: RouteResult) -> Self {
        match result {
            Ok(mut route_info) => Msg::Navigate(Route::from_route_main(&mut route_info)),
            Err(e) => {
                eprintln!("Couldn't route: {:?}", e);
                Msg::Navigate(Route::PageNotFound)
            }
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Route::Forums(ForumRoute::List)
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

        let callback = context.send_back(
            |route_result: RouteResult| {
                Msg::from(route_result)
            },
        );
        context.routing.register_router(
            callback,
        );


        let route: Route = Route::from_route_main(&mut context.routing.get_current_route_info());
        context.routing.replace_url(
            route.clone(),
        ); // sets the url to be dependent on what the route_info was resolved to

        Model { route }
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

        let page = |route: &Route| match route {
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
                            {forum_list_route.view() }
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
        };

        let header_links = vec![
            HeaderLink {
                name: "Forum".into(),
                link: Route::Forums(ForumRoute::List),
            },
            HeaderLink {
                name: "Login".into(),
                link: Route::Auth(AuthRoute::Login),
            },
        ];
        html! {
            <div class="main-container", >
                <Header: links=header_links, />
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

    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
