
use identifiers::forum::ForumUuid;
use identifiers::thread::ThreadUuid;

use yew::services::route::*;
#[derive(Debug, PartialEq, Clone)]
pub enum ForumRoute {
    ForumList,
    Forum{forum_id: ForumUuid},
    Thread {
        forum_id: ForumUuid,
        thread_id: ThreadUuid
    },
    CreateThread {
        forum_id: ForumUuid
    }
}

impl Default for ForumRoute {
    fn default() -> Self {
        ForumRoute::ForumList
    }
}


impl Router for ForumRoute {
    fn to_route(&self) -> RouteInfo {
        match *self {
            ForumRoute::ForumList => RouteInfo::parse("/").unwrap(),
            ForumRoute::Forum{forum_id} => {
                RouteInfo::parse(&format!("/{}", forum_id)).unwrap()
            }
            ForumRoute::Thread {forum_id, thread_id} => {
                RouteInfo::parse(&format!("/{}/{}", forum_id, thread_id)).unwrap()
            }
            ForumRoute::CreateThread {forum_id} => {
                RouteInfo::parse(&format!("/{}/create", forum_id)).unwrap()
            }
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node { segment }) = route.next() {
            if let Ok(forum_id) = ForumUuid::parse_str(&segment) {
                if let Some(RouteSection::Node {segment}) = route.next() {
                    if &segment == "create" {
                        Some(ForumRoute::CreateThread {forum_id})
                    } else if let Ok(thread_id) = ThreadUuid::parse_str(&segment) {
                        Some(ForumRoute::Thread {forum_id, thread_id})
                    } else {
                        None
                    }
                } else {
                    Some(ForumRoute::Forum{forum_id})
                }
            } else {
                Some(ForumRoute::ForumList) //TODO not sure about either this one or the one below
            }
        } else {
            Some(ForumRoute::ForumList)
        }
    }
}
