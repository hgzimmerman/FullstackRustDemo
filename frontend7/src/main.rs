//#![feature(try_from)]
#![recursion_limit="128"]

#[macro_use]
extern crate yew;
extern crate wire;
#[macro_use]
extern crate failure_derive;
extern crate failure;
extern crate serde;
//#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

#[macro_use]
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

mod util;


use yew::services::route::*;


use components::auth::AuthRoute;
use components::forum::ForumRoute;
use components::bucket::BucketRoute;
use components::auth::Auth;
use components::header_component::*;
use components::forum::ForumModel;
use components::bucket::BucketModel;



#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Forums(ForumRoute),
    //    ArticleView,
    Auth(AuthRoute),
    Bucket(BucketRoute),
    PageNotFound,
}


impl Router for Route {
    fn to_route(&self) -> RouteInfo {
        match *self {
            Route::Forums(ref forum_list_route) => RouteInfo::parse("/forum").unwrap() + forum_list_route.to_route(),
            Route::Auth(ref auth_route) => RouteInfo::parse("/auth").unwrap() + auth_route.to_route(),
            Route::Bucket(ref bucket_route) => RouteInfo::parse("/bucket").unwrap() + bucket_route.to_route(),
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
                "bucket" => {
                    if let Some(child) = BucketRoute::from_route(route) {
                        Route::Bucket(child)
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
            Err(_e) => {
//                eprintln!("Couldn't route: {:?}", e);
                Msg::Navigate(Route::PageNotFound)
            }
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Route::Forums(ForumRoute::ForumList)
    }
}


struct Model {
    route: Route,
    is_logged_in: bool
}


enum Msg {
    /// This should not be called by children, as the actions preformed due to this message don't affect the router state.
    /// This should only be called by the router logic itself.
    Navigate(Route),
    UpdateLogin,
    /// This can be called by children te sot the route
    #[allow(dead_code)]
    SetRoute(Route)
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

        Model { route, is_logged_in: context.is_logged_in() }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {

        match msg {
            Msg::Navigate(route) => {
                self.route = route;
                self.is_logged_in = context.is_logged_in(); // TODO remove this in the future.
                true
            }
            Msg::UpdateLogin => {
                // TODO This does not work reliably, so it is done on updates
                // TODO a good solution is implementing a Redux like solution by allowing anything with access to a context to send messages to this.
                self.is_logged_in = context.is_logged_in();
                true
            }
            Msg::SetRoute(route) => {
                context.routing.set_route(route);
                false // let the call back to ::Navigate do the updating
            }
        }
    }
}


impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {

        use self::Route::*;
        let page = |route: &Route| match route {
            Auth(ref auth_page) => {
                auth_page.view()
            }
            Forums(ref forum_list_route) => html! {
                <ForumModel: route=forum_list_route, />
            },
            Bucket(ref bucket_route) => html! {
//                bucket_route.view(),
                <BucketModel: route=bucket_route, />
            },
            PageNotFound => html! {
                <>
                    {"Page Not Found"}
                </>
            }
        };

        html! {
            <div class="main-container", >
                <Header: is_logged_in=self.is_logged_in,  />
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
