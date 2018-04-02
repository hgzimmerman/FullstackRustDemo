use yew::prelude::*;
use Context;

use components::button::Button;

pub enum State {
    Editing,
    RenderingMarkdown
}

pub struct AuthorMarkdownToggle {
    text: String,
    editor_state: State,
    callback: Option<Callback<String>>
}


pub enum Msg {
    UpdateText(String),
    ChangeState(State)
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub callback: Option<Callback<String>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            text: String::default(),
            callback: None
        }
    }
}

impl Component<Context> for AuthorMarkdownToggle {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        AuthorMarkdownToggle {
            text: props.text,
            editor_state: State::Editing,
            callback: props.callback
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::UpdateText(t) => {
                self.text = t.clone();
                if let Some(ref mut cb) = self.callback {
                    cb.emit(t);
                }
                true
            }
            Msg::ChangeState(state) => {
                self.editor_state = state;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        false
    }
}

impl Renderable<Context, AuthorMarkdownToggle> for AuthorMarkdownToggle {


    fn view(&self) -> Html<Context, Self> {


        let view = || match self.editor_state {
            State::Editing => html! {
                <>
                    <textarea
                        class=("markdown-textarea","form-control"),
                        value=&self.text,
                        oninput=|e: InputData| Msg::UpdateText(e.value),
                    />
                </>
            },
            State::RenderingMarkdown => html! {
               <div class="view-markdown-content",>
                    {super::render_markdown::<Context, Self>(&self.text)}
               </div>
            }
        };

        return html! {
            <div class="edit-markdown-toggle-box", >
                <div class="edit-markdown-bar",>
                    <Button: title="Edit", onclick=|_| Msg::ChangeState(State::Editing), />
                    <Button: title="View", onclick=|_| Msg::ChangeState(State::RenderingMarkdown), />
                </div>
                <div class="markdown-min-height",>
                    {view()}
                </div>

//                <div class="edit-markdown-bar",>
//                    <Button: title="Submit", disabled=false, onclick=|_| Msg::Submit, />
//                </div>
            </div>
        }
    }
}