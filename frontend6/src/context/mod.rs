pub mod route_service;
use self::route_service::RouteService;
pub mod networking;
pub mod storage;
pub mod role;

use yew::services::fetch::FetchService;
use yew::services::storage::{StorageService, Area};

pub struct Context {
    // Don't expose networking field. Make implementation use the Context's networking module instead.
    networking: FetchService,
    pub routing: RouteService,
    local_storage: StorageService,
    // console: ConsoleService,
}

impl Context {
    pub fn new() -> Context {
        Context {
            networking: FetchService::new(),
            routing: RouteService::new(),
            local_storage: StorageService::new(Area::Local)
        }
    }
}