use yew::prelude::*;
use Context;
use yew::format::{Json, Nothing};

use yew::services::fetch::Response;
use yew::services::fetch::Request;

use requests_and_responses::forum::ForumResponse;

use datatypes::forum::ForumData;
use components::link::Link;


pub struct ForumCardComponent {
    forum_data: ForumData,
    callback: Option<Callback<ForumData>>
}


pub enum Msg {
    Clicked
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub forum_data: ForumData,
    pub callback: Option<Callback<ForumData>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            forum_data: ForumData::default(),
            callback: None
        }
    }
}

impl Component<Context> for ForumCardComponent {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        ForumCardComponent {
            forum_data: props.forum_data,
            callback: props.callback
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

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, ForumCardComponent> for ForumCardComponent {

    fn view(&self) -> Html<Context, Self> {

        return html! {
            <>
                <div>
                    <Link: name=&self.forum_data.title, callback=|_| Msg::Clicked, classes="", />
                    {&self.forum_data.title}
                    {&self.forum_data.description}
                </div>
            </>
        }
    }
}