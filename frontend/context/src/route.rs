//! This module contains the implementation of a service for
//! setting the url and responding to changes to the url
//! that are initiated by the browser..

use stdweb::web::{History, Location, window};
use stdweb::Value;

use stdweb::web::{EventListenerHandle, IEventTarget};
use stdweb::web::event::PopStateEvent;
use stdweb::unstable::TryFrom;
use yew::callback::Callback;

use url::{Url};
use std::ops::Add;
use std::usize;


///// An alias for `Result<String, RoutingError>`.
//pub type RouteResult = Result<String, RouterError>;

/// Service used for routing.
pub struct RouteService {
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
    callback: Option<Callback<RouteResult>>
}


/// An error that can occur in the course of routing
#[derive(Debug, Clone, PartialEq)]
pub enum RouterError {
    CouldNotGetLocationHref
}

/// An alias for `Result<String, RoutingError>`.
pub type RouteResult = Result<String, RoutingError>;

/// An error that can occur in the course of routing
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingError {
    /// An error indicating that the string passed to the `RouteInfo::parse()` function couldn't parse the url.
    CouldNotParseRoute {
        /// In the event that url crate can't parse the route string, the route string will be passed back to the crate user to use.
        route: String
    },
    /// If the full Url can't be parsed this will be returned
    CouldNotParseUrl {
        /// This will contain the full url, not just the route.
        full_url: String
    },
    /// An error indicating that the string passed to the `RouteInfo::parse()` function did not start with a slash.
    RouteDoesNotStartWithSlash,
    /// An error indicating that the string passed to the `RouteInfo::parse()` function did not contain ary characters
    RouteIsEmpty,
    /// Indicates that the url could not be retrieved from the Location API.
    CouldNotGetLocationHref
}


impl RouteService {

    /// Creates a new route service
    pub fn new() -> RouteService {
        RouteService {
            history: window().history(),
            location: window().location().expect("Could not find Location API, routing is not supported in your browser."),
            event_listener: None,
            callback: None
        }
    }

    /// Clones the route service.
    /// This facilitates creating copies of the route service so that it may be moved into callbacks that aren't attached to components.
    pub fn clone_without_listener(&self) -> Self {
        RouteService {
            history: self.history.clone(),
            location: self.location.clone(),
            event_listener: None,
            callback: self.callback.clone(),
        }
    }


//    /// Will return the current route info based on the location API.
//    // TODO this should probably return a RouteResult and avoid expecting
    pub fn get_current_route(&mut self) -> String {
        // If the location api errors, recover by redirecting to a valid address
        let href = self.get_location().expect("Couldn't get href from location Api");
//        let url = Url::parse(&href).expect("The href returned from the location api should always be parsable.");
//        RouteInfo::from(url)
        href
    }

    /// Registers the router.
    /// There can only be one router.
    /// The component in which it is set up will be the source from which routing logic emanates.
    pub fn register_router(&mut self, callback: Callback<RouteResult>)
    {
        if let Some(_) = self.event_listener {
            panic!("You cannot register two separate routers.");
        }

        // Hold on to the callback so it can be used to update the main router component
        // when a user clicks a link, independent of the event listener.
        self.callback = Some(callback.clone());

        // Set the event listener to listen for the history's pop state events and call the callback when that occurs
        self.event_listener = Some( window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();

            if let Ok(state) = String::try_from(state_value) {
                callback.emit(Ok(state))
            } else {
//                eprintln!("Nothing farther back in history, not calling routing callback.");
            }
        }));
    }


//    /// Sets the route via the history api.
//    /// If the route is not already set to the string corresponding to the provided RouteInfo,
//    /// the history will be updated, and the routing callback will be invoked.
//    pub fn set_route<T: Router>(&mut self, r: T) {
//        let route_info: RouteInfo = r.to_route();
//        if route_info != self.get_current_route_info() {
//            let route_string: String = route_info.to_string();
//            println!("Setting route: {}", route_string); // this line needs to be removed eventually
//            let r = js! {
//                return @{route_string.clone()}
//            };
//            // Set the state using the history API
//            self.history.push_state(r, "", Some(&route_string));
//            self.go_to_current_route();
//        }
//    }

    /// Set the route using a string instead of something that implements a router.
    pub fn set_route(&mut self, r: String) {
        let route_string: String = r;
        println!("Setting route: {}", route_string); // this line needs to be removed eventually
        let r = js! {
            return @{route_string.clone()}
        };
        // Set the state using the history API
        self.history.push_state(r, "", Some(&route_string));
        self.go_to_current_route();
    }


    /// Replaces the url with the one provided by the route info.
    /// This will not invoke the routing callback.
    pub fn replace_url(&mut self, r: String) {
        let route_string: String = r;
        let r = js! {
            return @{route_string.clone()}
        };
        let _ = self.history.replace_state(r, "", Some(&route_string));
    }

    /// Based on the location API, set the route by calling the callback.
    pub fn go_to_current_route(&mut self) {
        if let Some(ref cb) = self.callback {

            let route_result: RouteResult = self.get_location();
            cb.emit(route_result)

        } else {
            eprintln!("Callback was never set.")
        }
    }

    /// Gets the location.
    pub fn get_location(&self) -> RouteResult {
        self.location.href().map_err(|_|RoutingError::CouldNotGetLocationHref)
    }
}