use yew::prelude::*;



pub struct Link {
    pub callback: Option<Callback<()>>,
    pub name: String,
    pub classes: String,
}


pub enum Msg {
    Clicked
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub callback:  Option<Callback<()>>,
    pub name: String,
    pub classes: String,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            callback: None,
            name: "".to_string(),
            classes: "".to_string()
        }
    }
}

impl<CTX: 'static> Component<CTX> for Link {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _:&mut Env<CTX, Self>) -> Self {
        Link {
            callback: props.callback,
            name: props.name,
            classes: props.classes
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut cb) = self.callback {
                    cb.emit(())
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.callback = props.callback;
        self.name = props.name;
        self.classes = props.classes;

        true
    }
}

impl<CTX: 'static> Renderable<CTX, Link> for Link {

    fn view(&self) -> Html<CTX, Self> {
        html!{
            <a onclick= |_| Msg::Clicked, >
                {&self.name}
            </a>
        }
    }
}




