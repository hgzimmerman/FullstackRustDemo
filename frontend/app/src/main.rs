//! This is the "binary" crate for the frontend;
//! the crate that when built produces the WASM needed to run the app.
//! The actual contents of this project should be kept to a minimum,
//! as it must be rebuilt whenever _any_ other frontend crate is changed.

#![recursion_limit="128"]

#[macro_use]
extern crate yew;
use yew::prelude::*;

extern crate context;
pub use context::Context;
pub use context::datatypes;
extern crate routes;

//extern crate util;

extern crate bucket;
extern crate forum;
extern crate auth;
extern crate header;

use routes::routing::MainRouter;
use context::route::RouteResult;
use routes::Route;
use routes::routing::RouteInfo;
use routes::routing::Router;

use header::Header;
use forum::ForumModel;
use bucket::BucketModel;
use auth::AuthModel;



impl From<RouteResult> for Msg {
    fn from(result: RouteResult) -> Self {
        match result {
            Ok(route_string) => {
                if let Ok(mut route_info) = RouteInfo::parse(&route_string) {
                    Msg::Navigate(Route::from_route_main(&mut route_info))
                } else {
                    Msg::Navigate(Route::PageNotFound)
                }
            },
            Err(_e) => {
//                eprintln!("Couldn't route: {:?}", e);
                Msg::Navigate(Route::PageNotFound)
            }
        }
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
    #[allow(dead_code)]
    UpdateLogin,
    /// This can be called by children te sot the route
    #[allow(dead_code)]
    SetRoute(Route)
}


impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        use yew::services::console::ConsoleService;
        let callback = context.send_back(
            |route_result: RouteResult| {
                let mut cs = ConsoleService::new();
                if let Ok(route) = route_result.clone() {
                    cs.log(&route)
                }
                Msg::from(route_result)
            },
        );
        context.routing.register_router(
            callback,
        );

        let current_location = context.routing.get_location().expect("Couldn't get location");
        let route: Route = if let Ok(mut current_route_info) = RouteInfo::parse(&current_location) {
            Route::from_route_main(&mut current_route_info)
        } else {
            Route::PageNotFound
        };

        context.routing.replace_url(
            route.clone().to_route().to_string(),
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
                context.routing.set_route(route.to_route().to_string());
                false // let the call back to ::Navigate do the updating
            }
        }
    }
}


impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {

        use self::Route::*;
        let page = |route: &Route| match route {
            Auth(ref auth_route) => html! {
                <AuthModel: route=auth_route, />
            },
            Forums(ref forum_list_route) => html! {
                <ForumModel: route=forum_list_route, />
            },
            Bucket(ref bucket_route) => html! {
                <BucketModel: route=bucket_route, />
            },
            PageNotFound => html! {
                <div>
                    {"Page Not Found"}
                </div>
            }
//                util::wrappers::html_string("Page Not Found".into())
        };

        html! {
            <div class="main-container", >
                // Apparently, the header needs to be wrapped in a div to preserve ordering.
                <div>
                    <Header: is_logged_in=self.is_logged_in, />
                </div>
                <div class="main-content", >
                    {page(&self.route)}
                </div>
            <div/>
        }
    }
}




fn main() {
    yew::initialize();
//    stdweb::initialize(); // I need this in order to use my route service
    let context = Context::new();

    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
