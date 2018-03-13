use threadCardComponent::ThreadCard;
use datatypes::minimal_thread::MinimalThread;
use Context;
use yew::prelude::*;


struct ForumModel {
    title: String,
    threads: Vec<MinimalThread>
}

enum Msg {
    Repaint,
}

impl Component<Context> for ForumModel {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        ForumModel {
            title: "Joe".into(),
            threads: vec!(
                MinimalThread {
                    id: 1,
                    title: "Why Does The Whole Women's Team Hate Me?".into(),
                    author: "Joe".into(),
                    replies: 6,
                    locked: false
                },
                MinimalThread {
                    id: 2,
                    title: "But I love him".into(),
                    author: "Emma".into(),
                    replies: 1,
                    locked: false
                },
                MinimalThread {
                    id: 4,
                    title: "But I LOVE HIM".into(),
                    author: "Emma".into(),
                    replies: 0,
                    locked: false
                },
            )
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Repaint => {
                // self.color = Color::Blue;
                true
            }
            // Msg::Toggle => {
            //     self.with_barrier = !self.with_barrier;
            //     true
            // }
            // Msg::ChildClicked(value) => {
            //     context.console.log(&format!("child clicked: {}", value));
            //     false
            // }
        }
    }
}


impl Renderable<Context, ForumModel> for ForumModel {
    fn view(&self) -> Html<Context, Self> {
        let thread_card = |x: &MinimalThread| html! {
            <ThreadCard: thread = x.clone(),/>
        };

        html! {
            <>
                <div class="main-container",>
                    <div class="thread-list", >
                        {for self.threads.iter().map(thread_card) }
                    </div>
                </div>
            </>
        }
    }
}
