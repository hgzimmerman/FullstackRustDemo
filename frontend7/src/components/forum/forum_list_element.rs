use yew::prelude::*;
use Context;

use datatypes::forum::ForumData;
use components::link::Link;


pub struct ForumListElement {
    forum_data: ForumData,
    callback: Option<Callback<ForumData>>,
}


pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub forum_data: ForumData,
    pub callback: Option<Callback<ForumData>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            forum_data: ForumData::default(),
            callback: None,
        }
    }
}

impl Component<Context> for ForumListElement {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {

        ForumListElement {
            forum_data: props.forum_data,
            callback: props.callback,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref cb) = self.callback {
                    cb.emit(self.forum_data.clone());
                }
                false

            }
        }
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, ForumListElement> for ForumListElement {
    fn view(&self) -> Html<Context, Self> {

        return html! {
            <li class="forum-list-element",>
                <div>
                    <Link<()>: name=&self.forum_data.title, callback=|_| Msg::Clicked, classes="forum-link", />
                </div>
                <div>
                    {&self.forum_data.description}
                </div>
            </li>
        };
    }
}
