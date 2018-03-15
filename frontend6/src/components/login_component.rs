use yew::prelude::*;
use Context;
use components::button::*;

use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use failure::Error;
use requests_and_responses::login::*;
use serde_json;

pub struct Login {
    user_name: String,
    password: String,
    ft: Option<FetchTask>,
}


pub enum Msg {
    UpdatePassword(String),
    UpdateUserName(String),
    Submit,
    NoOp
}


impl Component<Context> for Login {

    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Login {
            user_name: String::from(""),
            password: String::from(""),
            ft: None
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
                let login_request: LoginRequest = LoginRequest {
                    user_name: self.user_name.clone(),
                    password: self.password.clone()
                };
                let body = serde_json::to_string(&login_request).unwrap();
                let request = Request::post("http://localhost:8001/api/auth/login")
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
            Msg::UpdateUserName(u) => {
                self.user_name = u;
                true
            }
            Msg::NoOp => false
        }
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
//                <Button: title=&self.button_title, color=Color::Success, disabled=self.disabled, onclick=|_| Msg::Submit, />
            <div/>
        }

    }

}
