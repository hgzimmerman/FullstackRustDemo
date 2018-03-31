use yew::prelude::*;
use Context;
use yew::format::{Json, Nothing};

use yew::services::fetch::Response;
use yew::services::fetch::Request;

use requests_and_responses::forum::ForumResponse;


use components::link::Link;
use components::markdown::author_markdown_toggle::AuthorMarkdownToggle;
use components::button::Button;


pub struct NewThread {
    title: String,
    post_content: String,
    callback: Option<Callback<()>>
}


pub enum Msg {
    CreateNewThread,
    UpdatePostContent(String),
    UpdateThreadTitle(String)
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
            callback: props.callback
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::CreateNewThread => {
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