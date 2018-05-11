//use self::route_service::RouteService;
pub mod networking;
pub mod storage;
pub mod user;

use yew::services::fetch::FetchService;
use yew::services::storage::{StorageService, Area};
use yew::services::route::RouteService;
use yew::services::console::ConsoleService;

pub struct Context {
    /// Don't expose networking field. Make implementation use the Context's networking module instead.
    networking: FetchService,
    pub routing: RouteService,
    local_storage: StorageService,
    console: ConsoleService
}

impl Context {
    pub fn new() -> Context {
        Context {
            networking: FetchService::new(),
            routing: RouteService::new(),
            local_storage: StorageService::new(Area::Local),
            console: ConsoleService::new()
        }
    }
    pub fn log(&mut self, msg: &str) {
        self.console.log(msg)
    }
}
