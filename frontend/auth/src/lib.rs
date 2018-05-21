#[macro_use]
extern crate yew;
extern crate failure;
extern crate context;
extern crate wire;
extern crate util;
extern crate routes;

pub use context::datatypes;
pub use context::Context;
pub use routes::auth::AuthRoute;
pub use routes::Route;

pub mod login_component;
pub mod create_account_component;

use yew::prelude::*;
use self::login_component::Login;
use self::create_account_component::CreateAccount;
//use Context;


pub struct AuthModel {
    pub route: AuthRoute,
//    pub on_login_cb: Option<Callback<()>>
}

pub enum Msg {
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub route: AuthRoute,
    pub on_login_cb: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props { route: AuthRoute::Login, on_login_cb: None }
    }
}


impl Component<Context> for AuthModel {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        AuthModel {
            route: props.route,
//            on_login_cb: props.on_login_cb };
        }

    }

    fn update(&mut self, _msg: Self::Msg, _context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _context: &mut Env<Context, Self>) -> ShouldRender {
        self.route = props.route;
//        self.on_login_cb = props.on_login_cb;
        true
    }
}

impl Renderable<Context, AuthModel> for AuthModel {
    fn view(&self) -> Html<Context, Self> {

        match &self.route {
            &AuthRoute::Login => html! {
                <>
                    <Login: />
                </>
            },
            &AuthRoute::Create => html! {
                <>
                    <CreateAccount:  />
                </>
            }
        }
    }
}

//
//impl  Renderable<Context, Component<Context>> for AuthRoute
////    where
////        CMP: Component<Context>
//{
//    fn view(&self) -> Html<Context, Component<Context>> {
//
//        match self.route {
//            AuthRoute::Login => {
//                html! {
//                    <>
//                        <Login:  />
//                    </>
//                }
//            }
//            AuthRoute::Create => {
//                html! {
//                    <>
//                        <CreateAccount:  />
//                    </>
//                }
//            }
//        }
//
//    }
//}
