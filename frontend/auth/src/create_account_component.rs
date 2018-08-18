use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterButton;
//use Context;
use common::fetch::Networking;
use util::button::*;

//use yew::format::Json;
//use yew::services::fetch::{Response};
//use failure::Error;
use wire::user::*;

//use context::networking::*;

//use Route;
//use super::AuthRoute;

use util::uploadable::Uploadable;

use util::input::InputState;
use util::input::Input;
use util::input::InputValidator;

use common::fetch::FetchResponse;
use requests::AuthRequest;

//use routes::routing::Router;

pub enum Msg {
    UpdatePassword(InputState),
    UpdateConfirmPassword(InputState),
    UpdateUserName(InputState),
    UpdateDisplayName(InputState),
    Submit,
    NavigateToLogin,
    AccountCreationFailed,
    RequestStarted,
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
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
    networking: Networking,
    data: Uploadable<CreateAccountData>,
    link: ComponentLink<CreateAccount>
}

impl Routable for CreateAccount {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if let Some(seg_2) = route.path_segments.get(1) {
            if seg_2 == "create" {
                return Some(())
            }
        }
        return None
    }
    fn will_try_to_route(route: &Route) -> bool {
        if let Some(seg_1) = route.path_segments.get(0) {
            seg_1.as_str() == "auth"
        } else {
            false
        }
    }
}


impl Component for CreateAccount {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateAccount {
            networking: Networking::new(&link),
            data: Uploadable::default(),
            link
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
//                println!("Logging in with user name: {}", self.user_name);
                fn response_mapper(fetch_response: FetchResponse<()>) -> Msg { // TODO, I don't actually know this return type
                    match fetch_response {
                        FetchResponse::Started => Msg::RequestStarted,
                        FetchResponse::Success(_) => Msg::NavigateToLogin,
                        FetchResponse::Error(_) => Msg::AccountCreationFailed
                    }
                }

                match self.data.cloned_inner().validate()  {
                    Ok(new_user_request) => {
                        let request = AuthRequest::CreateUser(new_user_request);
                        self.networking.fetch(&request, response_mapper, &self.link );
                    }
                    Err(err_msg) => {
                        self.data.set_failed(err_msg);
//                        context.log("Couldn't validate create account data.")
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
//                context.routing.set_route(Route::Auth(AuthRoute::Login).to_route().to_string()); // navigate back to login page
                false
            }
            Msg::AccountCreationFailed => {
                self.data.set_failed("Could not create account.");
                true
            }
            Msg::RequestStarted => {
                self.data.set_uploading();
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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

impl Renderable<CreateAccount> for CreateAccount {
    fn view(&self) -> Html<Self> {
        fn create_account_view(create_account: &CreateAccountData) -> Html<CreateAccount> {
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
                            <RouterButton: text="Back To Login", route=route!("/auth/login"), />
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
