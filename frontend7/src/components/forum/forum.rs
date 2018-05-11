use yew::prelude::*;
use Context;
use yew::format::Json;

use yew::services::fetch::Response;

use wire::thread::MinimalThreadResponse;
use wire::forum::ForumResponse;

use datatypes::forum::ForumData;
use datatypes::thread::MinimalThreadData;

use yew::services::fetch::FetchTask;
use failure::Error;

use context::networking::*;

use components::forum::thread::thread_list_element::ThreadListElement;
use components::button::Button;
use components::forum::thread::new_thread::NewThread;
use components;

use yew::services::route::RouteInfo;
use yew::services::route::Router;
use yew::services::route::RouteSection;

use forum::thread;
use wire::thread::{NewThreadRequest, ThreadResponse};
use wire::post::NewPostRequest;
use datatypes::thread::NewThreadData;
use forum::ForumRoute;
use Route;


pub struct Forum {
    forum_data: ForumData,
    threads: Vec<MinimalThreadData>,
    threads_ft: Option<FetchTask>,
    forum_ft: Option<FetchTask>,
}


pub enum Msg {
    ContentReady(Vec<MinimalThreadData>),
    ForumReady(ForumData),
    NavigateToThread{thread_id: i32}
}

#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub forum_id: i32,
}

impl Forum {
    fn get_forum(forum_id: i32, context: &mut Env<Context, Self>) -> FetchTask {
        let forum_callback = context.send_back(
            |response: Response<Json<Result<ForumResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::ForumReady(data.unwrap().into())
            },
        );

        let forum_task = context.make_request(
            RequestWrapper::GetForum {
                forum_id,
            },
            forum_callback,
        );
        forum_task.unwrap()
    }

    fn get_threads(forum_id: i32, context: &mut Env<Context, Self>) -> FetchTask {
        let threads_callback = context.send_back(
            |response: Response<Json<Result<Vec<MinimalThreadResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::ContentReady(
                    data.unwrap()
                        .into_iter()
                        .map(MinimalThreadData::from)
                        .collect(),
                )
            },
        );

        let threads_task = context.make_request(
            RequestWrapper::GetThreads {
                forum_id,
                page_index: 1,
            },
            threads_callback,
        );
        threads_task.unwrap()
    }
}

impl Component<Context> for Forum {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        let forum_ft = Self::get_forum(props.forum_id, context);


        let threads_ft = Self::get_threads(props.forum_id, context);


        Forum {
            forum_data: ForumData::default(),
            threads: vec![],
            threads_ft: Some(threads_ft),
            forum_ft: Some(forum_ft),
        }

    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::ContentReady(threads) => {
                self.threads = threads;
                true
            }
            Msg::ForumReady(forum_data) => {
                self.forum_data = forum_data;
                true
            }
            Msg::NavigateToThread{thread_id} => {
                context.routing.set_route(Route::Forums(ForumRoute::Thread{forum_id: self.forum_data.id, thread_id}));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        if props.forum_id != self.forum_data.id {
            let forum_ft = Self::get_forum(props.forum_id, context);
            self.forum_ft = Some(forum_ft);

            let threads_ft = Self::get_threads(props.forum_id, context);
            self.threads_ft = Some(threads_ft);

        };
        true
    }
}

impl Renderable<Context, Forum> for Forum {
    fn view(&self) -> Html<Context, Self> {

        let thread_element = |x: &MinimalThreadData| {
            html! {
                <ThreadListElement: thread_data=x, callback=|td: MinimalThreadData| Msg::NavigateToThread{thread_id: td.id}, />
            }
        };

        // Only show the button if there is no child element
/*        let create_thread_button = || if let ForumRoute::Forum = self.route {
            html! {
                <Button: onclick=|_| Msg::Navigate(ForumRoute::Thread(ThreadRoute::CreateThread)), title="Create Thread", />
            }
        } else {
            html! {<></>}
        };*/


        let inner_content = || {
            html! {
                <ul class=("forum-list"),>
                    { for self.threads.iter().map(thread_element) }
                </ul>
            }
        };

        html! {
            <div class="vertical-flexbox", >
                <div class="centered",>
                    <div class="forum-title",>
                        <span class="forum-title-span", >{&self.forum_data.title} </span>
//                        {create_thread_button()}
                    </div>
                    {inner_content()}
                </div>
            </div>
        }
    }
}
