pub mod login_component;
pub mod create_account_component;

use yew::prelude::*;
use self::login_component::Login;
use self::create_account_component::CreateAccount;
use Context;

use routing::*;

#[derive(Clone, PartialEq, Debug)]
pub enum AuthPage {
    Login,
    Create
}

impl Routable for AuthPage {
    fn route(path_components: Vec<String>) -> AuthPage {
        if let Some(first) = path_components.get(0) {
            println!("Routing Auth: path is '{}'", first);
            match first.as_str() {
                "login" => AuthPage::Login,
                "create" => AuthPage::Create,
                _ => AuthPage::Login // default to bucket questions
            }
        } else {
            AuthPage::Login
        }
    }
}

pub struct Auth {
    pub child: AuthPage,
    pub callback: Option<Callback<()>>
}


pub enum Msg {
    Callback,
    SetChild(AuthPage)
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub child: Router<AuthPage>,
    pub callback: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            child: Router::Route(AuthPage::Login),
            callback: None
        }
    }
}

impl Component<Context> for Auth {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut auth = Auth {
            child: AuthPage::Login,
            callback: props.callback
        };
        auth.update(Msg::SetChild(props.child.resolve_route()), context);
        auth

    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Callback => {
                if let Some(ref mut cb) = self.callback {
                    cb.emit(())
                }
                false
            }
            Msg::SetChild(child) => {
//                context.routing.pop_route();
                match child {
                    AuthPage::Create => context.routing.set_route("/auth/create"),
                    AuthPage::Login => context.routing.set_route("/auth/login")
                }
                self.child = child;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _context: &mut Env<Context, Self>) -> ShouldRender {
        self.callback = props.callback;
        self.child = props.child.resolve_route();
        true
    }
}

impl Renderable<Context, Auth> for Auth{

    fn view(&self) -> Html<Context, Self> {

        let page = || {
            match &self.child {
                &AuthPage::Login => {
                    html! {
                        <>
                            <Login: login_nav_cb=|_| Msg::Callback, create_account_nav_cb=|_| Msg::SetChild(AuthPage::Create), />
                        </>
                    }
                }
                &AuthPage::Create => {
                    html! {
                        <>
                            <CreateAccount: nav_cb=|_| Msg::SetChild(AuthPage::Login), />
                        </>
                    }
                }
            }
        };

        return html! {
            <>
                {page()}
            </>
        }
    }
}



