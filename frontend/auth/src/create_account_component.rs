use yew::prelude::*;
use Context;
use util::button::*;

use yew::format::Json;
use yew::services::fetch::{Response};
use failure::Error;
use wire::user::*;

use context::networking::*;

use Route;
use super::AuthRoute;

use util::uploadable::Uploadable;

use util::input::InputState;
use util::input::Input;
use util::input::InputValidator;

use routes::routing::Router;

pub enum Msg {
    UpdatePassword(InputState),
    UpdateConfirmPassword(InputState),
    UpdateUserName(InputState),
    UpdateDisplayName(InputState),
    Submit,
    NavigateToLogin,
    AccountCreationFailed,
}

#[derive(Debug, Clone, Default)]
pub struct CreateAccountData {
    user_name: InputState,
    display_name: InputState,
    password: InputState,
    confirm_password: InputState,
}

impl CreateAccountData {
    fn validate(&self) -> Result<NewUserRequest, &str> {
        if self.confirm_password != self.password {
            return Err("Passwords do not match")
        }

        let request = NewUserRequest {
            user_name: self.user_name.inner_text().clone(),
            display_name: self.display_name.inner_text().clone(),
            plaintext_password: self.password.inner_text().clone(),
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
                        context.make_request_and_set_ft(
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
                context.routing.set_route(Route::Auth(AuthRoute::Login).to_route().to_string()); // navigate back to login page
                false
            }
            Msg::AccountCreationFailed => {
                self.data.set_failed("Could not create account.");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

fn validate_user_name(user_name: String) -> Result<String,String> {
    if user_name.len() < 5 {
        return Err("User Name must be 5 or more characters.".into())
    }
    Ok(user_name)
}

fn validate_display_name(display_name: String) -> Result<String, String> {
    if display_name.len() < 5 {
        return Err("Display Name must be 5 or more characters.".into())
    }
    Ok(display_name)
}


fn validate_password(password: String) -> Result<String, String> {
    if password.len() < 8 {
        return Err("Password must be 8 or more characters.".into())
    }
    Ok(password)
}
//fn validate_confirm_password(password: String) -> Result<String, String> {
//    Ok(password)
//}

impl Renderable<Context, CreateAccount> for CreateAccount {
    fn view(&self) -> Html<Context, Self> {
        fn create_account_view(create_account: &CreateAccountData) -> Html<Context, CreateAccount> {
            html! {
                <div class=("flexbox","full-height"),>
                    <div class=("login-card", "flexbox-vert", "flexbox-center-item"),>
                        <div class="flexbox-child-grow",>
                            <h3>
                                {"Create Account"}
                            </h3>
                            <Input:
    //                            required=true,
                                placeholder="Display Name",
                                input_state=&create_account.display_name,
                                on_change=|a| Msg::UpdateDisplayName(a),
                                on_enter=|_| Msg::Submit,
                                validator=Box::new(validate_display_name as InputValidator),
                            />

                            <Input:
    //                            required=true,
                                placeholder="User Name",
                                input_state=&create_account.user_name,
                                on_change=|a| Msg::UpdateUserName(a),
                                on_enter=|_| Msg::Submit,
                                validator=Box::new(validate_user_name as InputValidator),
                            />

                            <Input:
    //                            required=true,
                                placeholder="Password",
                                input_state=&create_account.password,
                                on_change=|a| Msg::UpdatePassword(a),
                                on_enter=|_| Msg::Submit,
                                validator=Box::new(validate_password as InputValidator),
                                is_password=true,
                            />

                            <Input:
    //                            required=true,
                                placeholder="Confirm Password",
                                input_state=&create_account.confirm_password,
                                on_change=|a| Msg::UpdateConfirmPassword(a),
                                on_enter=|_| Msg::Submit,
                                validator=Box::new(validate_password as InputValidator),
                                is_password=true,
                            />

                        </div>
                        <div>
                            <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
                            <Button: title="Back To Login", disabled=false, onclick=|_| Msg::NavigateToLogin, />
                        </div>
                    </div>
                </div>
            }
        }
        html! {
            <div class=("full-height","scrollable", "flexbox"),>
                <div class="flexbox-center-item",>
                    {self.data.default_view(create_account_view)}
                </div>
            </div>
        }

    }
}
