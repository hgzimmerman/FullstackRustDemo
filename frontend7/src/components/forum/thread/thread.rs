use yew::prelude::*;
use datatypes::thread::ThreadData;
use yew::services::fetch::Response;
use yew::format::Json;
use failure::Error;
use yew::services::fetch::FetchTask;
use context::Context;
use wire::thread::ThreadResponse;
use context::networking::RequestWrapper;
use components::forum::thread::post::PostTree;

pub struct Thread {
    thread_data: ThreadData,
}

pub enum Msg {
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub thread_data: ThreadData
}


impl Component<Context> for Thread {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        Thread {
            thread_data: props.thread_data,
        }

    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        self.thread_data = props.thread_data;
        true
    }
}

impl Renderable<Context, Thread> for Thread {
    fn view(&self) -> Html<Context,Self> {

        html! {
            <div>
                <PostTree: post=&self.thread_data.posts, />
            </div>
        }

    }
}