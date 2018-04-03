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

mod routing;
use routing::*;


//use yew::context::fetch::{FetchService, FetchTask, Request, Response};

use auth::AuthPage;
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
    ForumView(Router<ForumListRoute>),
    ArticleView,
    AuthView(Router<AuthPage>),
    BucketView,
}

impl Default for Route {
    fn default() -> Self {
        Route::ForumView(Router::Route(ForumListRoute::List))
    }
}


struct Model {
    page: Route,
}


enum Msg {
    Navigate(Route),
}

impl Routable for Route {
    fn route(path_components: Vec<String>) -> Route {

        println!("Routing Main: Routing with following path: {:?}", path_components);
        // The route is given in the form "/path/path/path"
        // The string at index 0 is "" because of the first "/", so get at index 1 here
        if let Some(first) = path_components.get(1) {
            println!("Routing Main: path is '{}'", first);
            match first.as_str() {
                "auth" => Route::AuthView(Router::Path(path_components[2..].to_vec())),
                "forum" => Route::ForumView(Router::Path(path_components[2..].to_vec())),
                "article" => Route::ArticleView,
                "bucket" => Route::BucketView,
                _ => Route::BucketView // default to bucket questions
            }
        } else {
            println!("Main router couldn't resolve route, setting default route");
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
                println!("MainNav: navigating to {:?}", route);
                self.page = route;
                true
            }
        }
    }
}




impl Renderable<Context, Model> for Model {

    fn view(&self) -> Html<Context, Self> {
        println!("Rendering main");

        let page = |page: &Route| {
            match page {
                Route::AuthView(ref auth_page) => {
                    html! {
                        <>
                            <Auth: child=auth_page, callback=|_| Msg::Navigate(Route::ForumView(Router::Route(ForumListRoute::List))), />
                        </>
                    }
                }
                Route::ForumView(ref forum_list_route) => {
                    println!("ForumView chosen to render by main with parameters {:?}", forum_list_route);
                    html! {
                        <>
                            <ForumList: route=forum_list_route.clone(), />
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
                link: Route::ForumView(Router::Route(ForumListRoute::List))
            },
            HeaderLink {
                name: "Login".into(),
                link: Route::AuthView(Router::Route(AuthPage::Login))
            },
        ];
        html! {
            <div class="main-container", >
                <Header: links=header_links, callback=|pv| Msg::Navigate(pv), />
                <div class="main-content", >
                    {page(&self.page)}
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
