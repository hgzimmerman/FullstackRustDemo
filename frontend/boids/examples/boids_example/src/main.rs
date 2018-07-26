
extern crate yew;
extern crate boids;
extern crate web_logger;
#[macro_use]
extern crate log;

use boids::BoidsModel;
use yew::app::App;

fn main() {
    web_logger::init();
    info!("Starting Application");
    yew::initialize();

    let app: App<BoidsModel> = App::new();
    app.mount_to_body();
    yew::run_loop();
}