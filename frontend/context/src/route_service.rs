use stdweb::web::History;
use stdweb::web::Location;
use stdweb::web::window;
//use stdweb::web::Window;
//use stdweb;
use stdweb::Value;
//use stdweb::Once;
use stdweb::web::EventListenerHandle;
use stdweb::web::event::PopStateEvent;
use stdweb::web::IEventTarget;
//use std::convert::TryInto;
//use std::collections::BTreeMap;
use stdweb::unstable::TryInto;


use yew::html::Callback;

pub struct RouteService {
    route: String,
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
}


impl RouteService {
    pub fn new() -> RouteService {
        RouteService {
            route: "".into(),
            history: window().history(),
            location: window().location().unwrap(),
            event_listener: None,
        }
    }

    pub fn register_callback(&mut self, callback: Callback<String>) {
        println!("Registering callback");
        self.event_listener = Some(window().add_event_listener(
            move |event: PopStateEvent| {
                let state_value: Value = event.state();
                if let Ok(prior_route) = state_value.try_into() {
                    println!("url changed: {}", prior_route);
                    println!(
                        "{}",
                        window()
                            .location()
                            .unwrap()
                            .href()
                            .unwrap()
                    );
                    callback.emit(prior_route)
                } else {
                    eprintln!("Nothing farther back in history, not calling routing callback.");
                }
            },
        ));

    }

    pub fn set_route(&mut self, route: &str) {
        println!("setting route: {}", route);
        let r =
            js! {
            return @{route.to_string()}
        };
        self.route = route.to_string();
        self.history.push_state(
            r,
            "",
            Some(&self.route),
        ); //.unwrap();
    }

    pub fn get_location(&self) -> String {
        self.location.href().unwrap()
    }

    pub fn get_route(&self) -> String {
        return self.route.clone();
    }
}
