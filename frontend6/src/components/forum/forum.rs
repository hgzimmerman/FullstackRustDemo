use yew::prelude::*;
use Context;
use yew::format::{Json};

use yew::services::fetch::Response;

use requests_and_responses::thread::MinimalThreadResponse;

use datatypes::forum::ForumData;
use datatypes::thread::MinimalThreadData;

use yew::services::fetch::FetchTask;
use failure::Error;

use context::networking::*;

use components::forum::thread::thread_list_element::ThreadListElement;
use components::button::Button;
use components::forum::thread::new_thread::NewThread;


#[derive(Clone, PartialEq)]
pub enum Child {
    CreateThread,
    ThreadContents(MinimalThreadData)
}

pub struct Forum {
    parent: ForumData,
    child: Option<Child>,
    threads: Vec<MinimalThreadData>,
    ft: Option<FetchTask>
}


pub enum Msg {
    ContentReady(Vec<MinimalThreadData>),
    SetChild(Child)
}

#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub forum_data: ForumData
}

impl Component<Context> for Forum {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        let callback = context.send_back(|response: Response<Json<Result<Vec<MinimalThreadResponse>, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            println!("META: {:?}, {:?}", meta, data);
            Msg::ContentReady(data.unwrap().into_iter().map(MinimalThreadData::from).collect())
        });

        let task = context.make_request(RequestWrapper::GetThreads{forum_id: props.forum_data.id, page_index: 1}, callback);

        Forum {
            parent: props.forum_data,
            child: None,
            threads: vec!(),
            ft: task.ok()
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetChild(child) => {
                self.child = Some(child);
                true
            }
            Msg::ContentReady(threads) => {
                self.threads = threads;
                self.ft = None;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, Forum> for Forum {

    fn view(&self) -> Html<Context, Self> {

        let thread_element = |x: &MinimalThreadData| html! {
            <ThreadListElement: thread_data=x, callback=|td| Msg::SetChild(Child::ThreadContents(td)), />
        };

        // Only show the button if there is no child element
        let create_thread_button = || {
            if let None = self.child {
               html! {
                    <Button: onclick=|_| Msg::SetChild(Child::CreateThread), title="Create Thread", />
                }
            } else {
                html! {<></>}
            }
        };

        let inner_content = || if let Some(ref child) = self.child {
            match child {
                &Child::CreateThread => {
                    html! {
                        <NewThread: />
                    }

                },
                &Child::ThreadContents(ref _minimal_thread_data) => {
                    html! {
                        <>
                            {"Inside of thread, a bunch of posts and stuff"}
                        </>
                    }
                }
            }
        }
        // No children, just show the threads for the current forum.
        else {
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
                        <span class="forum-title-span", >{&self.parent.title} </span>
                        {create_thread_button()}
                    </div>
                    {inner_content()}
                </div>
            </div>

        }

    }
}