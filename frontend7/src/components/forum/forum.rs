use yew::prelude::*;
use Context;
use yew::format::Json;

use yew::services::fetch::Response;

use requests_and_responses::thread::MinimalThreadResponse;

use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;
use datatypes::thread::MinimalThreadData;

use yew::services::fetch::FetchTask;
use failure::Error;

use context::networking::*;

use components::forum::thread::thread_list_element::ThreadListElement;
use components::button::Button;
use components::forum::thread::new_thread::NewThread;
use components;
use components::forum::thread::ThreadRoute;

use yew::services::route::RouteInfo;
use yew::services::route::Router;
use yew::services::route::RouteSection;

use forum::thread;
use requests_and_responses::thread::{NewThreadRequest, ThreadResponse};
use requests_and_responses::post::NewPostRequest;
use datatypes::thread::PartialNewThreadData;

#[derive(Clone, PartialEq, Debug)]
pub enum ForumRoute {
    Forum,
    Thread(ThreadRoute),
}

impl Default for ForumRoute {
    fn default() -> Self {
        ForumRoute::Forum
    }
}


impl Router for ForumRoute {
    fn to_route(&self) -> RouteInfo {
        match *self {
            ForumRoute::Forum => RouteInfo::parse("/").unwrap(),
            ForumRoute::Thread(ref thread_route) => RouteInfo::parse("/thread").unwrap() + thread_route.to_route(),
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node { segment }) = route.next() {
            match segment.as_str() {
                "thread" => Some(ForumRoute::Thread(ThreadRoute::from_route(route)?)),
                _ => Some(ForumRoute::Forum),
            }
        } else {
            None
        }
    }
}



pub struct Forum {
    route: ForumRoute,
    forum_data: ForumData,
    threads: Vec<MinimalThreadData>,
    threads_ft: Option<FetchTask>,
    forum_ft: Option<FetchTask>,
    create_thread_ft: Option<FetchTask>
}


pub enum Msg {
    ContentReady(Vec<MinimalThreadData>),
    Navigate(ForumRoute),
    ForumReady(ForumData),
    CreateThread(PartialNewThreadData)
}

#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub route: ForumRoute,
    pub forum_id: i32,
}

impl Forum {
    fn get_forum(forum_id: i32, context: &mut Env<Context, Self>) -> FetchTask {
        let forum_callback = context.send_back(
            |response: Response<Json<Result<ForumResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::ForumReady(
                    data.unwrap().into()
                )
            },
        );

        let forum_task = context.make_request(
            RequestWrapper::GetForum  {
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


        if let ForumRoute::Forum = props.route {
            let threads_ft = Self::get_threads(props.forum_id, context);


            Forum {
                route: props.route,
                forum_data: ForumData::default(),
                threads: vec![],
                threads_ft: Some(threads_ft),
                forum_ft: Some(forum_ft),
                create_thread_ft: None
            }
        } else {
            Forum {
                route: props.route,
                forum_data: ForumData::default(),
                threads: vec![],
                threads_ft: None,
                forum_ft: Some(forum_ft),
                create_thread_ft: None
            }
        }

    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Navigate(route) => {
                self.route = route;
                true
            }
            Msg::ContentReady(threads) => {
                self.threads = threads;
                true
            }
            Msg::ForumReady(forum_data) => {
                self.forum_data = forum_data;
                true
            }
            Msg::CreateThread(new_thread_data) => {
                let callback = context.send_back(
                    |response: Response<Json<Result<ThreadResponse, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        println!("META: {:?}, {:?}", meta, data);
                        Msg::Navigate(ForumRoute::Forum)
                    },
                );

                let new_thread_request = new_thread_data.attach_forum_id(self.forum_data.id);

                let task = context.make_request(
                    RequestWrapper::CreateThread(
                        new_thread_request,
                    ),
                    callback,
                );
                self.create_thread_ft = task.ok();
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        if props.forum_id != self.forum_data.id {
            let forum_ft = Self::get_forum(props.forum_id, context);
            self.forum_ft = Some(forum_ft);

            if let ForumRoute::Forum = props.route {
                let threads_ft = Self::get_threads(props.forum_id, context);
                self.threads_ft = Some(threads_ft)
            }
        };
        true
    }
}

impl Renderable<Context, Forum> for Forum {
    fn view(&self) -> Html<Context, Self> {

        let thread_element = |x: &MinimalThreadData| {
            html! {
                <ThreadListElement: thread_data=x, callback=|td: MinimalThreadData| Msg::Navigate(ForumRoute::Thread(thread::ThreadRoute::Thread{thread_id: td.id})), />
            }
        };

        // Only show the button if there is no child element
        let create_thread_button = || if let ForumRoute::Forum = self.route {
            html! {
                // TODO Make a special Msg:: for going to create thread page
                <Button: onclick=|_| Msg::Navigate(ForumRoute::Thread(ThreadRoute::CreateThread)), title="Create Thread", />
            }
        } else {
            html! {<></>}
        };


        let inner_content = || {
            match self.route{
                ForumRoute::Forum => html! {
                    <ul class=("forum-list"),>
                        { for self.threads.iter().map(thread_element) }
                    </ul>
                },
                ForumRoute::Thread(ref thread_route) => html! {
                    <div>
                        {thread_route.view()}
                    </div>
                }
            }

        };

        html! {
            <div class="vertical-flexbox", >
                <div class="centered",>
                    <div class="forum-title",>
                        <span class="forum-title-span", >{&self.forum_data.title} </span>
                        {create_thread_button()}
                    </div>
                    {inner_content()}
                </div>
            </div>

        }

    }
}
