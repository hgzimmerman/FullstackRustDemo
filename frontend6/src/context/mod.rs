pub mod route_service;
use self::route_service::RouteService;
pub mod networking;
pub mod storage;

use yew::services::fetch::FetchService;
use yew::services::storage::{StorageService, Area};

pub struct Context {
    // console: ConsoleService,
    pub networking: FetchService,
    pub routing: RouteService,
    pub local_storage: StorageService
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