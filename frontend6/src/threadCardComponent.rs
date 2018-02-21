use yew::prelude::*;
// use button::Button;
use datatypes::minimal_thread::MinimalThread;

pub struct ThreadCard {
    thread: MinimalThread
}

pub enum Msg {
    // ChildClicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub thread: MinimalThread
}

impl Default for Props {
    fn default() -> Self {
        Props {
            thread: MinimalThread::default()
        }
    }
}


impl<CTX: 'static> Component<CTX> for ThreadCard {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        ThreadCard {
            thread: props.thread
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        // self.limit = props.limit;
        // self.onsignal = props.onsignal;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, ThreadCard> for ThreadCard {
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="thread-card",>
                <div class="thread-card-title",>
                    {self.thread.title.clone()}
                </div>
                <div class="thread-card-author-link",>
                    {format!("By: {}", self.thread.author.clone())}
                </div>
                <div class="thread-card-author-link",>
                    { format!("Replies: {}",self.thread.replies) }
                </div>
            </div>
        }
    }
}