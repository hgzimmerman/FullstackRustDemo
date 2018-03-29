use stdweb::web::History;
use stdweb::web::window;
use stdweb;

pub struct RouteService {
    pub route: String,
    history: History
}


impl RouteService {

    pub fn new() -> RouteService {
        RouteService {
            route: "".into(),
            history: window().history()
        }
    }

    pub fn set_route(&mut self, route: String) {
        self.history.push_state((), "", Some(&route));
        self.route = route;
    }
}

