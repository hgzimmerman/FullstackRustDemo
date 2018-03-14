use yew::prelude::*;
use Context;


pub struct Login {
    user_name: String,
    password: String,
}


pub enum Msg {
    UpdatePassword(String),
    UpdateUserName(String),
    Submit
}


impl Component<Context> for Login {

    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Login {
            user_name: String::from(""),
            password: String::from("")
        }
    }


    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Submit => {
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
        }
    }
}

impl Renderable<Context, Login> for Login {
    fn view(&self) -> Html<Context, Self> {
        html!{
            <>
                {"Dis da login page, user name, password, and social security number pls..."}
            </>
        }

    }

}
