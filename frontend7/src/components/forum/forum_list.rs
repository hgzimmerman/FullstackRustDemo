use yew::prelude::*;
use Context;
use yew::format::{Json };

use yew::services::fetch::Response;
use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;

use components::forum::forum::Forum;
use components::forum::forum_list_element::ForumListElement;

use context::networking::*;

use routing::*;
use routing::Routable;
use yew::services::fetch::FetchTask;
use failure::Error;

#[derive(Clone, PartialEq, Debug)]
pub enum ForumListRoute {
    List,
    Forum(i32)
}
impl Routable for ForumListRoute {
    fn route(path_components: Vec<String>) -> ForumListRoute {
        if let Some(first) = path_components.get(0) {
            println!("Routing ForumList: path is '{}'", first);
            if let Ok(id) = first.parse::<i32>() {
                ForumListRoute::Forum(id)
            } else {
                ForumListRoute::List
            }
        } else {
            ForumListRoute::List
        }
    }
}

pub enum Child {
    List(Vec<ForumData>),
    Forum(ForumData),
    None
}


pub struct ForumList {
//    pub child: Option<ForumData>,
//    pub forums: Vec<ForumData>,
    pub child: Child,
    ft: Option<FetchTask>
}


pub enum Msg {
    SetChild(Child)
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub route: Router<ForumListRoute>
}




impl Default for Props {
    fn default() -> Self {
        Props {
            route: Router::Route(ForumListRoute::List)
        }
    }
}

impl ForumList {
    fn handle_route(route: ForumListRoute, context: &mut Env<Context, Self>) -> Option<FetchTask> {
        let task = match route {
            ForumListRoute::List => {
                let callback = context.send_back(|response: Response<Json<Result<Vec<ForumResponse>, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    let forum_data_list: Vec<ForumData> = data.expect("Forum Data invalid").into_iter().map(ForumData::from).collect();

                    Msg::SetChild(Child::List(forum_data_list))
                });
                context.make_request(RequestWrapper::GetForums, callback)
            }
            ForumListRoute::Forum(id) => {
                let callback = context.send_back(|response: Response<Json<Result<ForumResponse, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    let forum_data = data.map(ForumData::from).expect("Forum Data invalid");

                    Msg::SetChild(Child::Forum(forum_data))
                });
                context.make_request(RequestWrapper::GetForum{forum_id: id}, callback)
            }
        };
        task.ok()
    }
}

impl Component<Context> for ForumList {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        println!("Creating forum list");
        ForumList {
            child: Child::None,
            ft: ForumList::handle_route(props.route.resolve_route(), context)
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetChild(child) => {
                println!("Setting child");
                match &child {
                    Child::List(_) => context.routing.set_route("/forum"),
                    Child::Forum(forum_data) => context.routing.set_route(format!("/forum/{}", forum_data.id).as_str()),
                    _ => {}
                }
                self.child = child;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        println!("Forum container change() called");
        self.ft = ForumList::handle_route(props.route.resolve_route(), context);
        true
    }
}

impl Renderable<Context, ForumList> for ForumList {

    fn view(&self) -> Html<Context, Self> {
        let forum_element = |x: &ForumData| {
            html! {
                <ForumListElement: forum_data=x, callback=|fd| Msg::SetChild(Child::Forum(fd)), />
            }
        };

        match self.child {
            Child::Forum(ref forum_data) => html!{
                <>
                    <Forum: forum_data=forum_data, />
                </>
            },
            Child::List(ref forum_list) => html!{
                <div class="vertical-flexbox", >
                    <div class="centered",>
                        <div class="forum-title",>
                            <span class="forum-title-span", >{"Forums"} </span>
                        </div>
                        <ul class=("forum-list"),>
                            { for forum_list.iter().map(forum_element) }
                        </ul>
                    </div>
                </div>
            },
            Child::None => html!{
                <>
                </>
            }
        }


    }
}