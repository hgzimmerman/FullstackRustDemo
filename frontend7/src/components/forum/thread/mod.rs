
pub mod thread_list_element;
pub mod new_thread;

mod post;


use components::forum::forum::Forum;
use Context;
use yew::html::Renderable;
use yew::html::Html;

use yew::services::route::Router;
use yew::services::route::RouteInfo;
use yew::services::route::RouteSection;

use forum::thread::new_thread::NewThread;


#[derive(Debug, PartialEq, Clone)]
pub enum ThreadRoute {
    CreateThread,
    Thread{ thread_id: i32},
}

impl Default for ThreadRoute {
    fn default() -> Self {
        ThreadRoute::Thread {thread_id: 0}
    }
}

impl Router for ThreadRoute {
    fn to_route(&self) -> RouteInfo {
        match *self {
            ThreadRoute::CreateThread => RouteInfo::parse("/create").unwrap(),
            ThreadRoute::Thread{thread_id} => {
                RouteInfo::parse(&format!("/{}", thread_id)).unwrap()
            }
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node { segment }) = route.next() {
            if let Ok(id) = segment.parse::<i32>() {
                Some(ThreadRoute::Thread {thread_id: id})
            } else if segment.as_str() == "create" {
                Some(ThreadRoute::CreateThread)
            } else {
                None
            }
        } else {
            None
        }
    }
}


impl Renderable<Context, Forum> for ThreadRoute {
    fn view(&self) -> Html<Context, Forum> {

        use components::forum::forum::Msg::CreateThread;
        match *self {
            ThreadRoute::CreateThread => html! {
                <>
                    <NewThread: callback=|new_thread_data| CreateThread(new_thread_data), />
                </>
            },
            ThreadRoute::Thread {thread_id} => html! {
                <>
//                    <Thread: />
                    {"Inside of thread, a bunch of posts and stuff"}
                </>
            }


        }
    }
}
