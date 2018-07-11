extern crate common;
extern crate failure;
extern crate identifiers;
#[macro_use]
extern crate log;
//extern crate routes;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate util;
extern crate wire;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate yew_router;

pub use common::datatypes;
use new_thread::NewThread;
use yew::html::Renderable;
use yew::prelude::*;
use yew_router::prelude::*;



mod post_tree;
//mod list_elements;
mod new_thread;
mod new_forum;
mod requests;
mod title;
mod forums_list;
mod threads_list;
mod thread;

use title::{ForumTitle, ForumsTitle};
use forums_list::ForumsList;
use threads_list::ThreadsList;
use thread::Thread;
use new_forum::NewForum;
//pub mod forum_list;
//mod forum;

pub struct ForumModel;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Props;


pub enum Msg {
}


impl Component for ForumModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ForumModel
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<ForumModel> for ForumModel {
    fn view(&self) -> Html<ForumModel> {
        html! {
            <div class=("flexbox-vert","full-height", "no-scroll"),>
                <div class="flexbox-horiz",>
                    <div class=("title-bar", "flexbox-center-vert"),> // Title bar
                        <YewRouter: routes=routes![ForumTitle, ForumsTitle], />
                    </div>
                </div>
                <div class=("flexbox-horiz", "full-height", "no-scroll"), > // Horizontal container
                    <div class=("vertical-expand", "list-background", "forum-list-width", "scrollable"),> // Vertical - list container
                        <YewRouter: routes=routes![ForumsList, ThreadsList], />
                    </div>
                    <div class=("vertical-expand", "full-width", "scrollable" ),> // Vertical - content container
                        <YewRouter: routes=routes![Thread, NewThread, NewForum], />

                    </div>
                </div>
            </div>
        }
    }
}

impl Routable for ForumModel {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if let Some(seg_1) = route.path_segments.get(0) {
            if seg_1.as_str() == "forum" {
                Some(Props)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn will_try_to_route(route: &Route) -> bool {
        if let Some(_) = route.path_segments.get(0) {
            true
        } else {
            false
        }
    }
}