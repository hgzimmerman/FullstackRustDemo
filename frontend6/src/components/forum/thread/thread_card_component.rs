use yew::prelude::*;
use Context;
use yew::format::{Json, Nothing};

use yew::services::fetch::Response;
use yew::services::fetch::Request;

use requests_and_responses::forum::ForumResponse;


use datatypes::thread::MinimalThreadData;
use components::link::Link;


pub struct ThreadCardComponent {
    thread_data: MinimalThreadData,
    callback: Option<Callback<MinimalThreadData>>
}


pub enum Msg {
    Clicked
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub thread_data: MinimalThreadData,
    pub callback: Option<Callback<MinimalThreadData>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            thread_data: MinimalThreadData::default(),
            callback: None
        }
    }
}

impl Component<Context> for ThreadCardComponent {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        ThreadCardComponent {
            thread_data: props.thread_data,
            callback: props.callback
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

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, ThreadCardComponent> for ThreadCardComponent {

    fn view(&self) -> Html<Context, Self> {

        return html! {
            <div class="thread-card",>
                <div class="thread-card-title",>
                    {&self.thread_data.title}
                </div>
                <div class="thread-card-author-link",>
                    {format!("By: {}", &self.thread_data.author.display_name)}
                </div>
                <div class="thread-card-author-link",>
//                    { format!("Replies: {}",self.thread_dat.replies) }
                </div>
            </div>
        }
    }
}