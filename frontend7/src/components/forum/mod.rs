
use yew::services::route::*;
use yew::html::Html;
use yew::html::Renderable;

use Context;
use Model;
use forum::forum::ForumRoute as InnerForumRoute;
use forum::forum::Forum;

use forum::forum_list::ForumList;



mod forum_list_element;
mod thread;
pub mod forum_list;
mod forum;



#[derive(Debug, PartialEq, Clone)]
pub enum ForumRoute {
    List,
    Forum(i32, InnerForumRoute), //forum id
}

impl Default for ForumRoute {
    fn default() -> Self {
        ForumRoute::List
    }
}

impl Router for ForumRoute {
    fn to_route(&self) -> RouteInfo {
        match *self {
            ForumRoute::List => RouteInfo::parse("/").unwrap(),
            ForumRoute::Forum(forum_id, ref route) => {
                RouteInfo::parse(&format!("/{}", forum_id))
                    .unwrap() + route.to_route()
            }
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node { segment }) = route.next() {
            if let Ok(id) = segment.parse::<i32>() {
                Some(ForumRoute::Forum(id, InnerForumRoute::from_route(route)?))
            } else {
                Some(ForumRoute::List)
            }
        } else {
            Some(ForumRoute::List) // TODO, not sure if I don't want a None
        }
    }
}


impl Renderable<Context, Model> for ForumRoute {
    fn view(&self) -> Html<Context, Model> {
        match *self {
            ForumRoute::List => {
                html! {
                <>
                    <ForumList: />
                </>
            }
            }
            ForumRoute::Forum(id, ref route) => {
                html! {
                <>
                    <Forum: route=route, forum_id=id, />
                </>
            }
            }

        }
    }
}
