//! This is the "binary" crate for the frontend;
//! the crate that when built produces the WASM needed to run the app.
//! The actual contents of this project should be kept to a minimum,
//! as it must be rebuilt whenever _any_ other frontend crate is changed.

#![recursion_limit="128"]

#[macro_use]
extern crate yew;
use yew::prelude::*;
#[macro_use]
extern crate yew_router;
use yew_router::prelude::*;


extern crate bucket;
extern crate forum;
extern crate auth;
extern crate header;

#[macro_use]
extern crate log;
extern crate web_logger;


use header::Header;
use forum::ForumModel;
use bucket::BucketModel;
use auth::{Login, CreateAccount};

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}


impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="main-container", >
                // Apparently, the header needs to be wrapped in a div to preserve ordering.
                <div>
                    <Header: />
                </div>
                <div class="main-content", >
                    <YewRouter: routes=routes![Login, CreateAccount, BucketModel, ForumModel], />
                </div>
            <div/>
        }
    }
}


fn main() {
    web_logger::init();
    info!("Starting Application");
    yew::initialize();

    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
