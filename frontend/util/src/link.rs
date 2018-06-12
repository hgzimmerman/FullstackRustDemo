use yew::prelude::*;



pub struct Link<T> {
    pub callback: Option<Callback<T>>,
    cb_value: T,
    pub name: String,
    pub classes: &'static str,
}


pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq)]
pub struct Props<T> {
    pub callback: Option<Callback<T>>,
    pub cb_value: T,
    pub name: String,
    pub classes: &'static str,
}

impl<T: Default> Default for Props<T> {
    fn default() -> Self {
        Props {
            callback: None,
            cb_value: T::default(),
            name: "".to_string(),
            classes: "",
        }
    }
}

impl<CTX: 'static, T> Component<CTX> for Link<T>
where
    T: 'static + Clone + PartialEq + Default,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Link {
            callback: props.callback,
            cb_value: props.cb_value,
            name: props.name,
            classes: props.classes,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut cb) = self.callback {
                    cb.emit(self.cb_value.clone())
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

impl<CTX: 'static, T> Renderable<CTX, Link<T>> for Link<T>
where
    T: 'static + Clone + PartialEq + Default,
{
    fn view(&self) -> Html<CTX, Self> {
        html!{
            <a onclick= |_| Msg::Clicked, class={self.classes}, >
                {&self.name}
            </a>
        }
    }
}
