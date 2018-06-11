
use identifiers::forum::ForumUuid;
use identifiers::thread::ThreadUuid;

use routing::*;


#[derive(Debug, PartialEq, Clone)]
pub enum ForumRoute {
    ForumList,
    Forum{forum_uuid: ForumUuid},
    Thread {
        forum_uuid: ForumUuid,
        thread_uuid: ThreadUuid
    },
    CreateThread {
        forum_uuid: ForumUuid
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
            ForumRoute::Forum{forum_uuid} => {
                RouteInfo::parse(&format!("/{}", forum_uuid)).unwrap()
            }
            ForumRoute::Thread {forum_uuid, thread_uuid} => {
                RouteInfo::parse(&format!("/{}/{}", forum_uuid, thread_uuid)).unwrap()
            }
            ForumRoute::CreateThread {forum_uuid} => {
                RouteInfo::parse(&format!("/{}/create", forum_uuid)).unwrap()
            }
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node { segment }) = route.next() {
            if let Ok(forum_uuid) = ForumUuid::parse_str(&segment) {
                if let Some(RouteSection::Node {segment}) = route.next() {
                    if &segment == "create" {
                        Some(ForumRoute::CreateThread {forum_uuid})
                    } else if let Ok(thread_uuid) = ThreadUuid::parse_str(&segment) {
                        Some(ForumRoute::Thread {forum_uuid, thread_uuid})
                    } else {
                        None
                    }
                } else {
                    Some(ForumRoute::Forum{forum_uuid})
                }
            } else {
                Some(ForumRoute::ForumList) //TODO not sure about either this one or the one below
            }
        } else {
            Some(ForumRoute::ForumList)
        }
    }
}
