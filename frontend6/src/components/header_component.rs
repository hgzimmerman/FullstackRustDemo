use yew::prelude::*;
// use button::Button;
use datatypes::minimal_thread::MinimalThread;

pub struct Header {
}

pub enum Msg {
    // ChildClicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
}

impl Default for Props {
    fn default() -> Self {
        Props {
        }
    }
}


impl<CTX: 'static> Component<CTX> for Header {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Header {
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        // self.limit = props.limit;
        // self.onsignal = props.onsignal;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Header> for Header {
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="header",>
                { "WeekendAtJoes.com" }
            </div>
        }
    }
}