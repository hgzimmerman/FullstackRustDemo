use yew::prelude::*;
use Context;
use yew::format::{Json, Nothing};

use yew::services::fetch::Response;
use yew::services::fetch::Request;

use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;
use datatypes::post::MinimalNewPostData;
use components::link::Link;
use components::button::Button;

pub enum State {
    Editing,
    RenderingMarkdown
}

pub struct AuthorMarkdownToggle {
    forum_data: ForumData,
    callback: Option<Callback<String>>,
    text: String,
    submit_button_name: String,
    editor_state: State
}


pub enum Msg {
    Submit,
    UpdateText(String)
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub forum_data: ForumData,
    pub callback: Option<Callback<String>>,
    pub submit_button_name: String
}

impl Default for Props {
    fn default() -> Self {
        Props {
            forum_data: ForumData::default(),
            callback: None,
            submit_button_name: "Submit".to_string()
        }
    }
}

impl Component<Context> for AuthorMarkdownToggle {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        AuthorMarkdownToggle {
            forum_data: props.forum_data,
            callback: props.callback,
            text: String::default(),
            submit_button_name: props.submit_button_name,
            editor_state: State::Editing
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Submit => {
                if let Some(ref cb) = self.callback {
                    cb.emit(self.text.clone());
                }
                false
            }
            Msg::UpdateText(t) => {
                self.text = t;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, AuthorMarkdownToggle> for AuthorMarkdownToggle {


    fn view(&self) -> Html<Context, Self> {

//        use components::markdown::markdown;

        let view = || match self.editor_state {
            State::Editing => html! {
                <>
                    <textarea
                        class="form-control",
                        value=&self.text,
                        oninput=|e: InputData| Msg::UpdateText(e.value),
                    />
                </>
            },
            State::RenderingMarkdown => html! {
               <>
                    {"Rendering markdown not implemented, here's the plain text instead:"}
                    {&self.text}
                    //{markdown::render_markdown::<Context, Self>(&self.text)}
               </>
            }
        };

        return html! {
            <div>
                {view()}
                <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
            </div>
        }
    }
}