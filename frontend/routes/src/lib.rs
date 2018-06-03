
extern crate yew;

use yew::services::route::*;
extern crate identifiers;

pub mod auth;
pub mod bucket;
pub mod forum;
pub mod routing;

//pub use auth;
//pub use bucket;
//pub use forum;

use auth::AuthRoute;
use bucket::BucketRoute;
use forum::ForumRoute;


#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Forums(ForumRoute),
    //    ArticleView,
    Auth(AuthRoute),
    Bucket(BucketRoute),
    PageNotFound,
}


impl Router for Route {
    fn to_route(&self) -> RouteInfo {
        match *self {
            Route::Forums(ref forum_list_route) => RouteInfo::parse("/forum").unwrap() + forum_list_route.to_route(),
            Route::Auth(ref auth_route) => RouteInfo::parse("/auth").unwrap() + auth_route.to_route(),
            Route::Bucket(ref bucket_route) => RouteInfo::parse("/bucket").unwrap() + bucket_route.to_route(),
            Route::PageNotFound => {
                RouteInfo::parse("/pagenotfound")
                    .unwrap()
            }
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        Some(Self::from_route_main(route))
    }
}

impl MainRouter for Route {
    fn from_route_main(route: &mut RouteInfo) -> Self {
        if let Some(RouteSection::Node { segment }) = route.next() {
            match segment.as_str() {
                "forum" => {
                    if let Some(child) = ForumRoute::from_route(route) {
                        Route::Forums(child)
                    } else {
                        Route::PageNotFound
                    }
                }
                "auth" => {
                    if let Some(child) = AuthRoute::from_route(route) {
                        Route::Auth(child)
                    } else {
                        Route::PageNotFound
                    }
                }
                "bucket" => {
                    if let Some(child) = BucketRoute::from_route(route) {
                        Route::Bucket(child)
                    } else {
                        Route::PageNotFound
                    }
                }
                _ => Route::PageNotFound,
            }
        } else {
            Route::PageNotFound
        }
    }
}




impl Default for Route {
    fn default() -> Self {
        Route::Forums(ForumRoute::ForumList)
    }
}