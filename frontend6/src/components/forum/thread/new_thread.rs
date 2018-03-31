use yew::prelude::*;
use Context;
use yew::format::{Json};

use yew::services::fetch::{FetchTask, Request, Response};

use components::markdown::author_markdown_toggle::AuthorMarkdownToggle;
use components::button::Button;

use requests_and_responses::thread::{NewThreadRequest, ThreadResponse};
use requests_and_responses::post::NewPostRequest;

use failure::Error;
use serde_json;



pub struct NewThread {
    title: String,
    post_content: String,
    callback: Option<Callback<()>>,
    ft: Option<FetchTask>
}


pub enum Msg {
    CreateNewThread,
    UpdatePostContent(String),
    UpdateThreadTitle(String),
    NoOp
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub callback: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            callback: None
        }
    }
}

impl Component<Context> for NewThread {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        NewThread {
            title: String::default(),
            post_content: String::default(),
            callback: props.callback,
            ft: None
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::CreateNewThread => {
                let callback = context.send_back(|response: Response<Json<Result<ThreadResponse, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    Msg::NoOp
                });
                let new_thread_request = NewThreadRequest {
                    forum_id: 0,
                    author_id: 0,
                    title: self.title.clone(),
                    post: NewPostRequest {
                        author_id: 0,
                        thread_id: 0,
                        parent_id: None,
                        content: self.post_content.clone(),
                    }
                };
                let body = serde_json::to_string(&new_thread_request).unwrap();
                let request = Request::post("http://localhost:8001/api/thread/create")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap();
                let task = context.networking.fetch(request, callback);
                self.ft = Some(task);


                if let Some(ref cb) = self.callback {
                    cb.emit(());
                }
                false
            }
            Msg::UpdateThreadTitle(title) => {
                self.title = title;
                true
            }
            Msg::UpdatePostContent(text) => {
                self.post_content = text;
                true
            }
            Msg:: NoOp => {
                false
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
                    value=&self.title,
                    oninput=|e: InputData| Msg::UpdateThreadTitle(e.value),
//                    onkeypress=|e: KeyData| {
//                        if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
//                    },
                 />
                 <AuthorMarkdownToggle: callback=|text| Msg::UpdatePostContent(text), />
                 <Button: onclick=|_| Msg::CreateNewThread, />

            </div>
        }
    }
}