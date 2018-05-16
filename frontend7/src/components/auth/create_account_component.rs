use yew::prelude::*;
use Context;
use components::button::*;

use yew::format::Json;
use yew::services::fetch::{Response};
use failure::Error;
use wire::user::*;

use context::networking::*;

use Route;
use super::AuthRoute;

use util::uploadable::Uploadable;

pub enum Msg {
    UpdatePassword(String),
    UpdateConfirmPassword(String),
    UpdateUserName(String),
    UpdateDisplayName(String),
    Submit,
    NavigateToLogin,
    AccountCreationFailed,
    NoOp
}

#[derive(Debug, Clone, Default)]
pub struct CreateAccountData {
    user_name: String,
    display_name: String,
    password: String,
    confirm_password: String,
}

impl CreateAccountData {
    fn validate(&self) -> Result<NewUserRequest, &str> {
        if self.user_name.len() < 5 {
            return Err("User Name must be 5 or more characters.")
        }
        if self.display_name.len() < 5 {
            return Err("Display Name must be 5 or more characters.")
        }
        if self.password.len() < 8 {
            return Err("Password must be 8 or more characters ")
        }
        if self.confirm_password != self.password {
            return Err("Passwords do not match")
        }

        let request = NewUserRequest {
            user_name: self.user_name.clone(),
            display_name: self.display_name.clone(),
            plaintext_password: self.password.clone(),
        };
        Ok(request)
    }
}

pub struct CreateAccount {
    data: Uploadable<CreateAccountData>
}


impl Component<Context> for CreateAccount {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        CreateAccount {
            data: Uploadable::default()
        }
    }


    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Submit => {
//                println!("Logging in with user name: {}", self.user_name);
                let callback = context.send_back(
                    |response: Response<Json<Result<String, Error>>>| {
                        let (meta, Json(_data)) = response.into_parts();
//                        println!("META: {:?}, {:?}", meta, data);

                        if meta.status.is_success() {
                            Msg::NavigateToLogin
                        } else {
                            Msg::AccountCreationFailed
                        }
                    },
                );

                match self.data.cloned_inner().validate()  {
                    Ok(new_user_request) => {
                        context.make_logoutable_request(
                            &mut self.data,
                            RequestWrapper::CreateUser(
                                new_user_request,
                            ),
                            callback,
                        );
                    }
                    Err(err_msg) => {
                        self.data.set_failed(err_msg);
                        context.log("Couldn't validate create account data.")
                    }
                }

                true
            }
            Msg::UpdatePassword(p) => {
                self.data.as_mut().password = p;
                true
            }
            Msg::UpdateConfirmPassword(p) => {
                self.data.as_mut().confirm_password = p;
                true
            }
            Msg::UpdateUserName(u) => {
                self.data.as_mut().user_name = u;
                true
            }
            Msg::UpdateDisplayName(u) => {
                self.data.as_mut().display_name = u;
                true
            }
            Msg::NavigateToLogin => {
                context.routing.set_route(Route::Auth(AuthRoute::Login)); // navigate back to login page
                false
            }
            Msg::AccountCreationFailed => {
                self.data.set_failed("Could not create account.");
                true
            }
            Msg::NoOp => {
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, CreateAccount> for CreateAccount {
    fn view(&self) -> Html<Context, Self> {
        fn create_account_view(create_account: &CreateAccountData) -> Html<Context, CreateAccount> {
            html! {
                <div class=("login-card", "flexbox-vert"),>
                    <div class="flexbox-child-grow",>
                        <h3>
                            {"Create Account"}
                        </h3>
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="User Name",
                            value=&create_account.user_name,
                            oninput=|e: InputData| Msg::UpdateUserName(e.value),
                            onkeypress=|e: KeyData| {
                                if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                        />
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="Display Name",
                            value=&create_account.display_name,
                            oninput=|e: InputData| Msg::UpdateDisplayName(e.value),
                            onkeypress=|e: KeyData| {
                                if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                        />
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="Password",
                            value=&create_account.password,
                            oninput=|e: InputData| Msg::UpdatePassword(e.value),
                            onkeypress=|e: KeyData| {
                                if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                        />
                        <input
                            class="form-control",
                        //    disabled=self.disabled,
                            placeholder="Confirm Password",
                            value=&create_account.confirm_password,
                            oninput=|e: InputData| Msg::UpdateConfirmPassword(e.value),
                            onkeypress=|e: KeyData| {
                                if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                            },
                        />

                    </div>
                    <div>
                        <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
                        <Button: title="Back To Login", disabled=false, onclick=|_| Msg::NavigateToLogin, />
                    </div>
                </div>
            }
        }
        html! {
            <div class="flexbox-center",>
                {self.data.default_view(create_account_view)}
            </div>
        }

    }
}
