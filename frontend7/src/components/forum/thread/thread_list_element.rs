use yew::prelude::*;
use Context;

use datatypes::thread::MinimalThreadData;
use components::link::Link;


pub struct ThreadListElement {
    thread_data: MinimalThreadData,
    callback: Option<Callback<MinimalThreadData>>,
}


pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub thread_data: MinimalThreadData,
    pub callback: Option<Callback<MinimalThreadData>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            thread_data: MinimalThreadData::default(),
            callback: None,
        }
    }
}

impl Component<Context> for ThreadListElement {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        ThreadListElement {
            thread_data: props.thread_data,
            callback: props.callback,
        }
    }


    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref cb) = self.callback {
                    cb.emit(self.thread_data.clone());
                }
                false

            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, ThreadListElement> for ThreadListElement {
    fn view(&self) -> Html<Context, Self> {

        return html! {
            <li class="forum-list-element",>
                <div>
                    <Link<()>: name=&self.thread_data.title, callback=|_| Msg::Clicked, classes="forum-link", />
                </div>
                <div>
                    {format!("By: {}", &self.thread_data.author.display_name)}
                </div>
            </li>
        };
    }
}
