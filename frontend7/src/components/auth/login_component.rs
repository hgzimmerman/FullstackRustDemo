use yew::prelude::*;
use Context;
use components::button::*;

use yew::services::fetch::{FetchTask, Response};
use failure::Error;
use requests_and_responses::login::*;
use context::networking::*;
use super::AuthRoute;
use Route;


pub enum Msg {
    UpdatePassword(String),
    UpdateUserName(String),
    Submit,
    NavToCreateAccount,
    LoginSuccess(String),
    NoOp,
    LoginError,
}

pub struct Login {
    user_name: String,
    password: String,
    ft: Option<FetchTask>,
    create_account_nav_cb: Option<Callback<()>>,
    login_nav_cb: Option<Callback<()>>,
}


#[derive(PartialEq, Clone)]
pub struct Props {
    pub login_nav_cb: Option<Callback<()>>,
    pub create_account_nav_cb: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            login_nav_cb: None,
            create_account_nav_cb: None,
        }
    }
}


impl Component<Context> for Login {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        Login {
            user_name: String::from(""),
            password: String::from(""),
            ft: None,
            create_account_nav_cb: props.create_account_nav_cb,
            login_nav_cb: props.login_nav_cb,
        }
    }


    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Submit => {
                let callback = context.send_back(
                    |response: Response<Result<String, Error>>| {
                        let (meta, jwt) = response.into_parts();
//                        println!("META: {:?}, JWT: {:?}", meta, jwt);
                        if let Ok(j) = jwt {
                            // TODO This Result doesn't appear to indicate for errors, use meta instead
                            Msg::LoginSuccess(j)
                        } else {
                            Msg::LoginError
                        }
                    },
                );
                let login_request: LoginRequest = LoginRequest {
                    user_name: self.user_name.clone(),
                    password: self.password.clone(),
                };

                let task = context.make_request(
                    RequestWrapper::Login(
                        login_request,
                    ),
                    callback,
                );
                // This conversion of Err to Some is ok here because make_request will not fail with these parameters
                self.ft = task.ok();

                false
            }
            Msg::NavToCreateAccount => {
//                println!("LoginComponent: navigating to create account");
                if let Some(ref mut cb) = self.create_account_nav_cb {
                    cb.emit(())
                }
                context.routing.set_route(Route::Auth(
                    AuthRoute::Create,
                ));

                true
            }
            Msg::UpdatePassword(p) => {
                self.password = p;
                true
            }
            Msg::UpdateUserName(u) => {
                self.user_name = u;
                true
            }
            Msg::LoginSuccess(jwt) => {
                context.store_jwt(jwt);
                if let Some(ref mut cb) = self.login_nav_cb {
                    cb.emit(())
                }
                true
            }
            Msg::LoginError => {
                //TODO, add an element indicating that the login failed
                true
            }
            Msg::NoOp => false,
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        //        self.nav_cb = props.nav_cb;33
        true
    }
}

impl Renderable<Context, Login> for Login {
    fn view(&self) -> Html<Context, Self> {
        html!{
            <div>
                {"Login"}
                <input
                    class="form-control",
                //    disabled=self.disabled,
                    placeholder="User Name",
                    value=&self.user_name,
                    oninput=|e: InputData| Msg::UpdateUserName(e.value),
                    onkeypress=|e: KeyData| {
                        if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                    },
                />
                <input
                    class="form-control",
                //    disabled=self.disabled,
                    placeholder="Password",
                    value=&self.password,
                    oninput=|e: InputData| Msg::UpdatePassword(e.value),
                    onkeypress=|e: KeyData| {
                        if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                    },
                />

                <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
                <Button: title="Create Account", disabled=false, onclick=|_| Msg::NavToCreateAccount, />
//                <Button: title=&self.button_title, color=Color::Success, disabled=self.disabled, onclick=|_| Msg::Submit, />

            <div/>
        }

    }
}
