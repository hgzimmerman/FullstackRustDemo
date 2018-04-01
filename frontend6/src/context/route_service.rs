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

pub struct RouteService {
    route: String,
    history: History,
    location: Location,
//    callback: UrlChangedCbHandle
    event_listener: Option<EventListenerHandle>,
//    callbacks: Vec<usize>
}


impl RouteService {

    pub fn new() -> RouteService {
        let mut route_service = RouteService {
            route: "".into(),
            history: window().history(),
            location: window().location().unwrap(),
            event_listener: None,
//            callbacks: vec![]
        };


        route_service.event_listener = Some(window().add_event_listener(|event: PopStateEvent| {
            let state_value: Value = event.state();
            let prior_route: String = state_value.try_into().unwrap();
            println!("url changed: {}", prior_route);
//            route_service.set_route(prior_route);
        }));

        route_service
    }

    pub fn set_route(&mut self, route: &str) {
        println!("setting route: {}", route);
        let r = js! {
            return @{route.to_string()}
        };
        self.route = route.to_string();
        self.history.push_state(r, "", Some(&self.route));//.unwrap();
    }

    pub fn get_location(&self) -> String {
        self.location.href().unwrap()
    }


//
//    pub fn push_route(&mut self, route_segment: &str) {
//        println!("pushing route: {}", route_segment);
//        let route: String = self.route.clone() + "/" + route_segment;
//        println!("new_total_route: {}", route);
//
//        self.route = route.clone();
//        let r = js! {
//            return @{route.clone()}
//        };
//        self.history.push_state(r, "", Some(&self.route))
//    }
//
//    pub fn pop_route(&mut self) {
//        println!("popping route");
//        let cloned_route = self.route.clone();
//        let mut new_route: Vec<&str> = cloned_route.split('/').collect();
//        let _ = new_route.pop();
//        let route = new_route.join("/");
//
//        self.set_route(&route);
//    }


    pub fn get_route(&self) -> String {
        return self.route.clone();
    }
}

