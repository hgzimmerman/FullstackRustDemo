use yew::prelude::*;
use Context;
use yew::format::Json;

use yew::services::fetch::Response;
use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;

use components::forum::forum_list_element::ForumListElement;

use context::networking::*;

//use routing::*;
//use routing::Routable;
use yew::services::route::*;
use yew::services::fetch::FetchTask;
use failure::Error;
use Route;
use forum::ForumRoute;
use forum::forum::ForumRoute as InnerForumRoute;


pub struct ForumList {
    pub ft: Option<FetchTask>,
    pub forum_list: Vec<ForumData>,
}


pub enum Msg {
    ContentReady(Vec<ForumData>),
    NavigateToSpecificForum(ForumData)
}

//impl ForumList {
//    fn handle_route(route: ForumListRoute, context: &mut Env<Context, Self>) -> Option<FetchTask> {
//        let task = match route {
//            ForumListRoute::List => {
//                let callback = context.send_back(|response: Response<Json<Result<Vec<ForumResponse>, Error>>>| {
//                    let (meta, Json(data)) = response.into_parts();
//                    println!("META: {:?}, {:?}", meta, data);
//                    let forum_data_list: Vec<ForumData> = data.expect("Forum Data invalid").into_iter().map(ForumData::from).collect();
//
//                    Msg::ContentReady(Child::List(forum_data_list))
//                });
//                context.make_request(RequestWrapper::GetForums, callback)
//            }
//            ForumListRoute::Forum(id, route) => {
//                let callback = context.send_back(|response: Response<Json<Result<ForumResponse, Error>>>| {
//                    let (meta, Json(data)) = response.into_parts();
//                    println!("META: {:?}, {:?}", meta, data);
//                    let forum_data = data.map(ForumData::from).expect("Forum Data invalid");
//
//                    Msg::ContentReady(Child::Forum(forum_data))
//                });
//                context.make_request(RequestWrapper::GetForum{forum_id: id}, callback)
//            }
//        };
//        task.ok()
//    }
//}

impl Component<Context> for ForumList {
    type Msg = Msg;
    type Properties = ();

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        println!("Creating forum list");

        let callback = context.send_back(
            |response: Response<Json<Result<Vec<ForumResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                let forum_data_list: Vec<ForumData> = data.expect("Forum Data invalid")
                    .into_iter()
                    .map(ForumData::from)
                    .collect();

                Msg::ContentReady(forum_data_list)
            },
        );

        let ft = context
            .make_request(RequestWrapper::GetForums, callback)
            .ok();

        ForumList {
            ft,
            forum_list: vec![],
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::ContentReady(list) => {

                self.forum_list = list;
                true
            }
            Msg::NavigateToSpecificForum(forum_data) => {
                context.routing.set_route(Route::Forums(ForumRoute::Forum(forum_data.id, InnerForumRoute::Forum)));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        println!("Forum container change() called");
        true
    }
}

impl Renderable<Context, ForumList> for ForumList {
    fn view(&self) -> Html<Context, Self> {

        let forum_element = |x: &ForumData| {
            html! {
                <ForumListElement: forum_data=x, callback=|fd| Msg::NavigateToSpecificForum(fd),/>
            }
        };

        html!{
            <div class="vertical-flexbox", >
                <div class="centered",>
                    <div class="forum-title",>
                        <span class="forum-title-span", >{"Forums"} </span>
                    </div>
                    <ul class=("forum-list"),>
                        { for self.forum_list.iter().map(forum_element) }
                    </ul>
                </div>
            </div>
        }
    }
}
