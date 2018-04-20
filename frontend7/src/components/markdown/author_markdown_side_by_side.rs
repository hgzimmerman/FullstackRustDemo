use yew::prelude::*;
use Context;

pub struct AuthorMarkdownSideBySide {
    text: String,
    callback: Option<Callback<String>>,
}


pub enum Msg {
    UpdateText(String),
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub callback: Option<Callback<String>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            text: String::default(),
            callback: None,
        }
    }
}

impl Component<Context> for AuthorMarkdownSideBySide {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        AuthorMarkdownSideBySide {
            text: props.text,
            callback: props.callback,
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
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        false
    }
}

impl Renderable<Context, AuthorMarkdownSideBySide> for AuthorMarkdownSideBySide {
    fn view(&self) -> Html<Context, Self> {

        return html! {
            <div class="edit-markdown-side-by-side-wrapper", >
                <div class=("edit-markdown-half", "border-right"),>
                    <textarea
                        class=("markdown-textarea","form-control"),
                        value=&self.text,
                        oninput=|e: InputData| Msg::UpdateText(e.value),
                    />
                </div>
                <div class="edit-markdown-half",>
                    {super::render_markdown::<Context, Self>(&self.text)}
                </div>
            </div>
        };
    }
}
