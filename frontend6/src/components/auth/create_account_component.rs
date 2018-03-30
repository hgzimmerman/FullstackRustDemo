use yew::prelude::*;
use Context;
use components::button::*;

use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use failure::Error;
use requests_and_responses::user::*;
use serde_json;

use PageView;


pub enum Msg {
    UpdatePassword(String),
    UpdateConfirmPassword(String),
    UpdateUserName(String),
    UpdateDisplayName(String),
    Submit,
    NoOp
}

pub struct CreateAccount {
    user_name: String,
    display_name: String,
    password: String,
    confirm_password: String,
    ft: Option<FetchTask>,
    nav_cb: Option<Callback<()>>
}


#[derive(PartialEq, Clone)]
pub struct Props {
    pub nav_cb: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            nav_cb: None
        }
    }
}


impl Component<Context> for CreateAccount {

    type Msg = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.routing.set_route("/auth/create");
        println!("location: {}",context.routing.get_location());

        CreateAccount {
            user_name: String::from(""),
            display_name: String::from(""),
            password: String::from(""),
            confirm_password: String::from(""),
            ft: None,
            nav_cb: None
        }
    }


    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Submit => {
                println!("Logging in with user name: {}", self.user_name);
                let callback = context.send_back(|response: Response<Json<Result<String, ()>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    Msg::NoOp
                });
                let new_user_request =  NewUserRequest {
                    user_name: self.user_name.clone(),
                    display_name: self.display_name.clone(),
                    plaintext_password: self.password.clone()
                };
                let body = serde_json::to_string(&new_user_request).unwrap();
                let request = Request::post("http://localhost:8001/api/user")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap();
                let task = context.networking.fetch(request, callback);
                self.ft = Some(task);
                false
            },
            Msg::UpdatePassword(p) => {
                self.password = p;
                true
            }
            Msg::UpdateConfirmPassword(p) => {
                self.confirm_password = p;
                true
            }
            Msg::UpdateUserName(u) => {
                self.user_name = u;
                true
            }
            Msg::UpdateDisplayName(u) => {
                self.display_name = u;
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        self.nav_cb = props.nav_cb;
        true
    }
}

impl Renderable<Context, CreateAccount> for CreateAccount {
    fn view(&self) -> Html<Context, Self> {
        html!{
            <div>
                {"Create Account"}

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
                    placeholder="Display Name",
                    value=&self.display_name,
                    oninput=|e: InputData| Msg::UpdateDisplayName(e.value),
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
                <input
                    class="form-control",
                //    disabled=self.disabled,
                    placeholder="Confirm Password",
                    value=&self.confirm_password,
                    oninput=|e: InputData| Msg::UpdateConfirmPassword(e.value),
                    onkeypress=|e: KeyData| {
                        if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
                    },
                />

                <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
//                <Button: title=&self.button_title, color=Color::Success, disabled=self.disabled, onclick=|_| Msg::Submit, />
            <div/>
        }

    }

}
