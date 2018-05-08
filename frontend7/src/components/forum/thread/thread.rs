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
    thread_data: Option<ThreadData>,
    ft: Option<FetchTask>,
}

pub enum Msg {
    ThreadReady(ThreadData)
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub thread_id: i32
}

impl Thread {
    fn get_thread(thread_id: i32, context: &mut Env<Context, Self>) -> FetchTask {
        let callback = context.send_back(
            |response: Response<Json<Result<ThreadResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::ThreadReady(data.unwrap().into())
            },
        );

        let forum_task = context.make_request(
            RequestWrapper::GetThread {
                thread_id,
            },
            callback,
        );
        forum_task.unwrap()
    }
}

impl Component<Context> for Thread {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let ft = Self::get_thread(props.thread_id, context);
        Thread {
            thread_data: None,
            ft: Some(ft)
        }

    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::ThreadReady(thread) => self.thread_data = Some(thread)
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, Thread> for Thread {
    fn view(&self) -> Html<Context,Self> {

        if let Some(ref thread) = self.thread_data {
            html! {
                <div>
                    <PostTree: post=&thread.posts, />
                </div>
            }

        } else {
            html! {
                <div>
                    {"Loading Thread"}
                </div>
            }
        }

    }
}