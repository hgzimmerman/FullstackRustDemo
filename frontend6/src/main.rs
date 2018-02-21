#[macro_use]
extern crate yew;
extern crate requests_and_responses;

// mod counter;
// mod button;
// mod barrier;

use yew::prelude::*;
use yew::html::Scope;
use yew::services::console::ConsoleService;
// use counter::{Counter, Color};
// use barrier::Barrier;
mod datatypes;
use datatypes::minimal_thread::MinimalThread;
mod threadCardComponent;
use threadCardComponent::ThreadCard;
mod header_component;
use header_component::Header;


struct Context {
    // console: ConsoleService,
}

/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
// impl counter::Printer for Context {
//     fn print(&mut self, data: &str) {
//         self.console.log(data);
//     }
// }

struct Model {
    title: String,
    threads: Vec<MinimalThread>
}

enum Msg {
    Repaint,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
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

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        // let counter = |_: | html! {
        //     <div> { "hi" } </div>
        //     // <Counter: initial=x, color=&self.color, onclick=Msg::ChildClicked,/>
        // };
        let thread_card = |x: &MinimalThread| html! {
            <ThreadCard: thread = x.clone(),/>
        };

        html! {
            <>
                <Header: />
                <div class="main-container",>
                    <div class="thread-list", > 
                        {for self.threads.iter().map(thread_card) }
                    </div>
                </div>
            </>
            // <div class="custom-components-example",>
            //     <button onclick=|_| Msg::Toggle,>{ "Toggle" }</button>
            //     { self.view_barrier() }
            //     { for (1..1001).map(counter) }
            // </div>
        }
    }
}

// impl Model {
//     fn view_barrier(&self) -> Html<Context, Self> {
//         // if self.with_barrier {
//         //     html! {
//         //         <Barrier: limit=10, onsignal=|_| Msg::Repaint, />
//         //     }
//         // } else {
//         //     html! {
//         //         <p>{ "Click \"toggle\"!" }</p>
//         //     }
//         // }
//     }
// }

fn main() {
    yew::initialize();
    let context = Context {
        // console: ConsoleService,
    };
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
