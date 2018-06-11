#[macro_use]
extern crate yew;
extern crate failure;
extern crate context;
//extern crate wire;
extern crate util;
extern crate routes;

use yew::prelude::*;
use util::link::Link;
use context::Context;

use routes::Route;
use routes::auth::AuthRoute;
use routes::forum::ForumRoute;
use routes::bucket::BucketRoute;

use routes::routing::Router;


#[derive(Clone, PartialEq)]
pub struct HeaderLink {
    pub link: Route,
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct Header {
//    pub links: Vec<HeaderLink>,
//    pub callback: Option<Callback<Route>>
    is_logged_in: bool
}

pub enum Msg {
//    CallLink(Route),
    Login,
    Logout,
    Forums,
    BucketQuestions
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub is_logged_in: bool
}

impl Default for Props {
    fn default() -> Self {
        Props {
            is_logged_in: false
        }
    }
}


impl Component<Context> for Header {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Header {
            is_logged_in: props.is_logged_in
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            Login => context.routing.set_route(Route::Auth(AuthRoute::Login).to_route().to_string()),
            Logout => {
                context.remove_jwt();
                self.is_logged_in = false;
                context.routing.set_route(Route::Auth(AuthRoute::Login).to_route().to_string());
            }
            Forums => context.routing.set_route(Route::Forums(ForumRoute::ForumList).to_route().to_string()),
            BucketQuestions => context.routing.set_route(Route::Bucket(BucketRoute::BucketList).to_route().to_string())
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        self.is_logged_in = props.is_logged_in;
        true
    }
}

impl Renderable<Context, Header> for Header {
    fn view(&self) -> Html<Context, Self> {

        let log_in_out = if self.is_logged_in {
            html! {
                <Link<()>: name="Logout", callback=|_| Msg::Logout, classes="nav-link", />
            }
        } else {
            html! {
                <Link<()>: name="Login", callback=|_| Msg::Login, classes="nav-link", />
            }
        };

        let bucket_questions = if self.is_logged_in {
            html! {
                <Link<()>: name="Bucket Questions", callback=|_| Msg::BucketQuestions, classes="nav-link", />
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
                        <Link<()>: name="Forums", callback=|_| Msg::Forums, classes="nav-link", />
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

