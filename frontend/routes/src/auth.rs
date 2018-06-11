
use routing::*;

#[derive(Clone, PartialEq, Debug)]
pub enum AuthRoute {
    Login,
    Create,
}


impl Router for AuthRoute {
    fn to_route(&self) -> RouteInfo {
        match *self {
            AuthRoute::Login => RouteInfo::parse("/login").unwrap(),
            AuthRoute::Create => RouteInfo::parse("/create").unwrap(),
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node { segment }) = route.next() {
            match segment.as_str() {
                "login" => Some(AuthRoute::Login),
                "create" => Some(AuthRoute::Create),
                _ => None,
            }
        } else {
            None
        }
    }
}


