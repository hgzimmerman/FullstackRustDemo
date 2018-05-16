use yew::prelude::*;
use Context;
use components::button::*;

use yew::services::fetch::{Response};
use failure::Error;
use wire::login::*;
use context::networking::*;
use super::AuthRoute;
use Route;
use util::uploadable::Uploadable;


pub enum Msg {
    UpdatePassword(String),
    UpdateUserName(String),
    Submit,
    NavToCreateAccount,
    LoginSuccess(String),
    NoOp,
    LoginError,
}

#[derive(Debug, Default, Clone)]
pub struct LoginData {
    user_name: String,
    password: String
}


pub struct Login {
    login_data: Uploadable<LoginData>,
    create_account_nav_cb: Option<Callback<()>>,
}


#[derive(PartialEq, Clone)]
pub struct Props {
    pub create_account_nav_cb: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            create_account_nav_cb: None,
        }
    }
}


impl Component<Context> for Login {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        Login {
            login_data: Uploadable::default(),
            create_account_nav_cb: props.create_account_nav_cb,
        }
    }


    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Submit => {
                let callback = context.send_back(
                    |response: Response<Result<String, Error>>| {
                        let (meta, jwt) = response.into_parts();
//                        println!("META: {:?}, JWT: {:?}", meta, jwt);

                        if meta.status.is_success() {
                            if let Ok(j) = jwt {
                                Msg::LoginSuccess(j)
                            } else {
                                Msg::LoginError
                            }
                        } else {
                            Msg::LoginError
                        }
                    },
                );

                let login_data = self.login_data.cloned_inner();

                let login_request: LoginRequest = LoginRequest {
                    user_name: login_data.user_name,
                    password: login_data.password,
                };

                context.make_logoutable_request(
                    &mut self.login_data,
                    RequestWrapper::Login(
                        login_request,
                    ),
                    callback,
                );

//                self.login_data.attach_fetch_task(task);
                true
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
                self.login_data.as_mut().password = p;
                true
            }
            Msg::UpdateUserName(u) => {
                self.login_data.as_mut().user_name = u;
                true
            }
            Msg::LoginSuccess(jwt) => {
                context.store_jwt(jwt); // store/upsert the local JWT.

                use Route;
                use components::forum::ForumRoute;
                context.routing.set_route(Route::Forums(ForumRoute::ForumList));
                true
            }
            Msg::LoginError => {
                self.login_data.set_failed("Login Failed, try another user name combo");
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
        fn login_view(login_data: &LoginData) -> Html<Context, Login> {
            html! {
                <div class=("login-card", "flexbox-vert"),>
                    <div class="flexbox-child-grow",>
                        <h3>
                            {"Login"}
                        </h3>
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="User Name",
                            value=&login_data.user_name,
                            oninput=|e: InputData| Msg::UpdateUserName(e.value),
                            onkeypress=|e: KeyData| {
                                if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                        />
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="Password",
                            value=&login_data.password,
                            oninput=|e: InputData| Msg::UpdatePassword(e.value),
                            onkeypress=|e: KeyData| {
                                if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                            type="password",
                        />
                    </div>

                    <div class=("flexbox-horiz"),>
                        <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
                        <Button: title="Create Account", disabled=false, onclick=|_| Msg::NavToCreateAccount, />
                    </div>
                </div>
            }
        }
        html! {

            <div class="flexbox-center",>
                {self.login_data.default_view(login_view)}
            </div>
        }

    }
}
