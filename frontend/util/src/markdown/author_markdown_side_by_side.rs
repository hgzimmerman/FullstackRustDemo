use yew::prelude::*;

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

impl Component for AuthorMarkdownSideBySide {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {

        AuthorMarkdownSideBySide {
            text: props.text,
            callback: props.callback,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<AuthorMarkdownSideBySide> for AuthorMarkdownSideBySide {
    fn view(&self) -> Html<Self> {

        return html! {
            <div class="edit-markdown-side-by-side-wrapper", >
                <div class=("edit-markdown-half", "border-right"),>
                    <textarea
                        class=("markdown-textarea","form-control"),
                        value=&self.text,
                        oninput=|e| Msg::UpdateText(e.value),
                    />
                </div>
                <div class="edit-markdown-half",>
                    {super::render_markdown::<Self>(&self.text)}
                </div>
            </div>
        };
    }
}
