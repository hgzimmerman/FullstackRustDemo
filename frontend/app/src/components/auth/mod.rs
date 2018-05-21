pub mod login_component;
pub mod create_account_component;

use yew::prelude::*;
use self::login_component::Login;
use self::create_account_component::CreateAccount;
use Context;

use routes::auth::AuthRoute;

pub struct Auth {
    pub child: AuthRoute,
    pub on_login_cb: Option<Callback<()>>
}

pub enum Msg {
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub child: AuthRoute,
    pub on_login_cb: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props { child: AuthRoute::Login, on_login_cb: None }
    }
}


// TODO, remove the component here, it doesn't offer anything
impl Component<Context> for Auth {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        let auth = Auth { child: props.child, on_login_cb: props.on_login_cb };
        //        auth.update(Msg::SetChild(props.child.resolve_route()), context);
        auth

    }

    fn update(&mut self, _msg: Self::Msg, _context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _context: &mut Env<Context, Self>) -> ShouldRender {
        self.child = props.child;
        self.on_login_cb = props.on_login_cb;
        true
    }
}

impl Renderable<Context, Auth> for Auth {
    fn view(&self) -> Html<Context, Self> {

        match &self.child {
            &AuthRoute::Login => {
                html! {
                        <>
                            <Login: on_login_cb=&self.on_login_cb, />
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
        }
    }
}

use Model;

impl Renderable<Context, Model> for AuthRoute {
    fn view(&self) -> Html<Context, Model> {

        match *self {
            AuthRoute::Login => {
                html! {
                        <>
                            <Login:  />
                        </>
                    }
            }
            AuthRoute::Create => {
                html! {
                        <>
                            <CreateAccount:  />
                        </>
                    }
            }
        }
    }
}
