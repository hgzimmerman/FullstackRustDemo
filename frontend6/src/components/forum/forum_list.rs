use yew::prelude::*;
use Context;
use yew::format::{Json, Nothing};

use yew::services::fetch::Response;
use yew::services::fetch::Request;

use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;

use components::forum::forum::Forum;
use components::forum::forum_list_element::ForumListElement;


use yew::services::fetch::FetchTask;

pub struct ForumList {
    pub child: Option<ForumData>,
    pub forums: Vec<ForumData>,
    ft: Option<FetchTask>
}


pub enum Msg {
    ContentReady(Vec<ForumData>),
    SetChild(ForumData)
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub child: Option<ForumData>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            child: None
        }
    }
}

impl Component<Context> for ForumList {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        let callback = context.send_back(|response: Response<Json<Result<Vec<ForumResponse>, ()>>>| {
            let (meta, Json(data)) = response.into_parts();
            println!("META: {:?}, {:?}", meta, data);
            Msg::ContentReady(data.expect("Forum Data invalid").into_iter().map(ForumData::from).collect())
        });
        let request = Request::get("http://localhost:8001/api/forum/forums")
            .header("Content-Type", "application/json")
            .body(Nothing)
            .unwrap();
        let task = context.networking.fetch(request, callback);

        ForumList {
            child: None,
            forums: vec!(),
            ft: Some(task)
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetChild(fd) => {
                println!("Setting child");
                self.child = Some(fd);
                true
            }
            Msg::ContentReady(forums) => {
                self.forums = forums;
                self.ft = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        println!("Forum container change() called");
        self.child = props.child;
        true
    }
}

impl Renderable<Context, ForumList> for ForumList {

    fn view(&self) -> Html<Context, Self> {
        let forum_element = |x: &ForumData| {
            html! {
                <ForumListElement: forum_data=x, callback=|fd| Msg::SetChild(fd), />
            }
        };

        return if let Some(ref child) = self.child {
            html!{
                <>
                    <Forum: forum_data=child, />
                </>
            }
        } else {
            html!{
                <div class="vertical-flexbox", >
                    <div class="centered",>
                        <div class="forum-title",>
                            <span class="forum-title-span", >{"Forums"} </span>
                        </div>
                        <ul class=("forum-list"),>
                            { for self.forums.iter().map(forum_element) }
                        </ul>
                    </div>
                </div>
            }
        }

    }
}