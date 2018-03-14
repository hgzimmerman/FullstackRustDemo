#[macro_use]
extern crate yew;
extern crate requests_and_responses;
extern crate failure;

// mod counter;
// mod button;
// mod barrier;

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

use yew::services::fetch::{FetchService, FetchTask, Request, Response};


struct Context {
    // console: ConsoleService,
    networking: FetchService
}

/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
// impl counter::Printer for Context {
//     fn print(&mut self, data: &str) {
//         self.console.log(data);
//     }
// }
//
enum PageView {
    ForumListView,
    ThreadListView,
    ThreadView,
    ArticleView,
    ArticleAuthoringView,
    LoginView,
    BucketSelectionView,
    BucketView,
    ChatView,
    AllChatsView
}

struct Model {
    page: PageView,
    jwt: Option<String> // Its dumb to store it here, but for now, this is where the jwt will live. Instead it should use the localstorage api.
}


enum Msg {
    Repaint,
    Login{ jwt: String },
    Logout
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            page: PageView::LoginView,
            jwt: None
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Repaint => {
                true
            }
            Msg::Login {jwt} => {
                // Set the jwt
                self.jwt = Some(jwt);
                true
            }
            Msg::Logout => {
                // Invalidate the JWT
                self.jwt = None;
                // Navigate elsewhere
                self.page = PageView::LoginView;
                true
            }
        }
    }
}

use components::login_component;

use components::login_component::Login;
impl Renderable<Context, Model> for Model {

    fn view(&self) -> Html<Context, Self> {


        let page = || {
            match self.page {
                PageView::LoginView => {
                    html! {
                        <>
                            <Login: />
                        </>
                    }
                }
                _ => {
                    html! {
                        <>
                            {"Not implemented"}
                        </>
                    }
                }
            }
        };

        html! {
            <>
                <Header: />
                {page()}
            </>
        }
    }
}



fn main() {
    yew::initialize();
    let context = Context {
        networking: FetchService::new()
    };
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
