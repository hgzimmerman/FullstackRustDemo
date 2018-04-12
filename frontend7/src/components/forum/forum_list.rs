use yew::prelude::*;
use Context;
use yew::format::{Json };

use yew::services::fetch::Response;
use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;

use components::forum::forum::Forum;
use components::forum::forum_list_element::ForumListElement;

use context::networking::*;

//use routing::*;
//use routing::Routable;
use yew::services::route::*;
use yew::services::fetch::FetchTask;
use failure::Error;

use forum::forum::ForumRoute;

#[derive(Clone, PartialEq, Debug)]
pub enum ForumListRoute {
    List(Vec<ForumData>),
    Forum(ForumRoute)
}

impl Default for ForumListRoute {
    fn default() -> Self {
        ForumListRoute::List(vec!())
    }
}


impl <'a> From<&'a RouteInfo> for ForumListRoute {
    fn from(route_info: &RouteInfo) -> Self {
        println!("Converting from url");
        if let Some(segment) = route_info.get_segment_at_index(1) {
            println!("matching: {}", segment);
            match segment {
                "" => return ForumListRoute::List(vec!()),
                "id" => return ForumListRoute::Forum(route_info.into()),
                _ => return ForumListRoute::default()
            }
        }
        ForumListRoute::default()
    }
}

impl Into<RouteInfo> for ForumListRoute {
    fn into(self) -> RouteInfo {
        match self {
            ForumListRoute::List(_) => RouteInfo::parse("/").unwrap(),
            ForumListRoute::Forum(route) => RouteInfo::parse("/create").unwrap() + route.into(),
        }
    }
}

//impl Routable for ForumListRoute {
//    fn route(path_components: Vec<String>) -> ForumListRoute {
//        if let Some(first) = path_components.get(0) {
//            println!("Routing ForumList: path is '{}'", first);
//            if let Ok(id) = first.parse::<i32>() {
//                ForumListRoute::Forum(id)
//            } else {
//                ForumListRoute::List
//            }
//        } else {
//            ForumListRoute::List
//        }
//    }
//}

pub struct ForumList {
    pub route: ForumListRoute,
}


pub enum Msg {
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub route: ForumListRoute,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            route: ForumListRoute::List(vec!())
        }
    }
}

//
//impl ForumList {
//    fn handle_route(route: ForumListRoute, context: &mut Env<Context, Self>) -> Option<FetchTask> {
//        let task = match route {
//            ForumListRoute::List => {
//                let callback = context.send_back(|response: Response<Json<Result<Vec<ForumResponse>, Error>>>| {
//                    let (meta, Json(data)) = response.into_parts();
//                    println!("META: {:?}, {:?}", meta, data);
//                    let forum_data_list: Vec<ForumData> = data.expect("Forum Data invalid").into_iter().map(ForumData::from).collect();
//
//                    Msg::SetChild(Child::List(forum_data_list))
//                });
//                context.make_request(RequestWrapper::GetForums, callback)
//            }
//            ForumListRoute::Forum(id) => {
//                let callback = context.send_back(|response: Response<Json<Result<ForumResponse, Error>>>| {
//                    let (meta, Json(data)) = response.into_parts();
//                    println!("META: {:?}, {:?}", meta, data);
//                    let forum_data = data.map(ForumData::from).expect("Forum Data invalid");
//
//                    Msg::SetChild(Child::Forum(forum_data))
//                });
//                context.make_request(RequestWrapper::GetForum{forum_id: id}, callback)
//            }
//        };
//        task.ok()
//    }
//}

impl Component<Context> for ForumList {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        println!("Creating forum list");
        ForumList {
            route: props.route,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
        }
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        println!("Forum container change() called");
//        self.ft = ForumList::handle_route(props.route.resolve_route(), context);
        true
    }
}

impl Renderable<Context, ForumList> for ForumList {

    fn view(&self) -> Html<Context, Self> {

        let forum_element = |x: &ForumData| {
            html! {
                <ForumListElement: forum_data=x, />
            }
        };

        match self.route {
            ForumListRoute::Forum(ref route) => html!{
                <>
                    <Forum: route=route, />
                </>
            },
            ForumListRoute::List(ref list_of_forums) => html!{
                <div class="vertical-flexbox", >
                    <div class="centered",>
                        <div class="forum-title",>
                            <span class="forum-title-span", >{"Forums"} </span>
                        </div>
                        // TODO this should be its own component
                        <ul class=("forum-list"),>
                            { for list_of_forums.iter().map(forum_element) }
                        </ul>
                    </div>
                </div>
            },
        }


    }
}