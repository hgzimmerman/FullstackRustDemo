pub mod login_component;
pub mod create_account_component;

use yew::prelude::*;
use self::login_component::Login;
use self::create_account_component::CreateAccount;
use Context;

use yew::services::route::*;

#[derive(Clone, PartialEq, Debug)]
pub enum AuthRoute {
    Login,
    Create,
}


//impl<'a> From<&'a RouteInfo> for AuthRoute {
//    fn from(route_info: &RouteInfo) -> Self {
//        println!("Converting from url");
//        if let Some(segment) = route_info.get_segment_at_index(1) {
//            println!("matching: {}", segment);
//            match segment {
//                "login" => return AuthRoute::Login,
//                "create" => return AuthRoute::Create,
//                _ => return AuthRoute::Login,
//            }
//        }
//        AuthRoute::Login
//    }
//}
//
//impl Into<RouteInfo> for AuthRoute {
//    fn into(self) -> RouteInfo {
//        match self {
//            AuthRoute::Login => RouteInfo::parse("/login").unwrap(),
//            AuthRoute::Create => RouteInfo::parse("/create").unwrap(),
//        }
//    }
//}

impl Router for AuthRoute {
    fn to_route(&self) -> RouteInfo {
        match *self {
            AuthRoute::Login => RouteInfo::parse("/login").unwrap(),
            AuthRoute::Create => RouteInfo::parse("/create").unwrap(),
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node{segment}) = route.next() {
            match segment.as_str() {
                "login" => Some(AuthRoute::Login),
                "create" => Some(AuthRoute::Create),
                _ => None
            }
        } else {
            None
        }
    }
}

pub struct Auth {
    pub child: AuthRoute,
}


pub enum Msg {
    SetChild(AuthRoute),
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub child: AuthRoute,
}

impl Default for Props {
    fn default() -> Self {
        Props { child: AuthRoute::Login }
    }
}

impl Component<Context> for Auth {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut auth = Auth { child: props.child };
        //        auth.update(Msg::SetChild(props.child.resolve_route()), context);
        auth

    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetChild(child) => {
                //                match child {
                //                    AuthRoute::Create => context.routing.set_route("/auth/create"),
                //                    AuthRoute::Login => context.routing.set_route("/auth/login")
                //                }
                self.child = child;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _context: &mut Env<Context, Self>) -> ShouldRender {
        self.child = props.child;
        true
    }
}

impl Renderable<Context, Auth> for Auth {
    fn view(&self) -> Html<Context, Self> {

        let page = || match &self.child {
            &AuthRoute::Login => {
                html! {
                        <>
                            <Login: />
                        </>
                    }
            }
            &AuthRoute::Create => {
                html! {
                        <>
                            <CreateAccount:  />
                        </>
                    }
            }
        };

        return html! {
            <>
                {page()}
            </>
        };
    }
}
