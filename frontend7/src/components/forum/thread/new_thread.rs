use yew::prelude::*;
use Context;

use components::markdown::author_markdown_toggle::AuthorMarkdownToggle;
use components::button::Button;

use wire::thread::{NewThreadRequest, ThreadResponse};
use wire::post::NewPostRequest;
use datatypes::forum::ForumData;
use failure::Error;

use context::networking::*;
use datatypes::thread::NewThreadData;
use datatypes::thread::ThreadData;
use yew::format::Json;
use yew::services::fetch::Response;
use yew::services::fetch::FetchTask;
use Route;
use forum::ForumRoute;


pub struct NewThread {
    new_thread: NewThreadData,
    create_thread_ft: Option<FetchTask>
}


pub enum Msg {
    CreateNewThread,
    UpdatePostContent(String),
    UpdateThreadTitle(String),
    NavigateToNewThread{new_thread_id: i32}
}

#[derive(Clone, PartialEq, Default)]
pub struct Props {
    pub new_thread: NewThreadData
}


impl Component<Context> for NewThread {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        NewThread {
            new_thread: props.new_thread,
            create_thread_ft: None
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            // Todo: maybe move the responsibility for uploading into the ForumModel
            Msg::CreateNewThread => {
                if let Ok(user_id) = context.user_id() {

                    let callback = context.send_back(
                        |response: Response<Json<Result<ThreadResponse, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            println!("META: {:?}, {:?}", meta, data);
                            let thread_data: ThreadData = data.unwrap().into();
                            Msg::NavigateToNewThread{new_thread_id: thread_data.id}
                        },
                    );

                    let new_thread_request: NewThreadRequest = self.new_thread.clone().into();

                    let task = context.make_request(
                        RequestWrapper::CreateThread(
                            new_thread_request,
                        ),
                        callback,
                    );
                    self.create_thread_ft = task.ok();

                } else {
                    eprintln!("Couldn't get user id required for request")
                }
                true
            }
            Msg::UpdateThreadTitle(title) => {
                self.new_thread.title = title;
                true
            }
            Msg::UpdatePostContent(text) => {
                self.new_thread.post_content = text;
                true
            }
            Msg::NavigateToNewThread {new_thread_id} =>  {
//                context.routing.set_route(Route::Forums(ForumRoute::Thread{forum_id: self.forum_id, thread_id: new_thread_id}));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, NewThread> for NewThread {
    fn view(&self) -> Html<Context, Self> {

        return html! {
            <div>
                <input
                    class="form-control",
                //    disabled=self.disabled,
                    placeholder="Thread Title",
                    value=&self.new_thread.title,
                    oninput=|e: InputData| Msg::UpdateThreadTitle(e.value),
//                    onkeypress=|e: KeyData| {
//                        if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
//                    },
                 />
                 <AuthorMarkdownToggle: callback=|text| Msg::UpdatePostContent(text), />
                 <Button: onclick=|_| Msg::CreateNewThread, />

            </div>
        };
    }
}
