use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
pub enum Router<T>
    where T: Routable +
        Debug +
        Clone
{
    Route(T),
    Path(Vec<String>)
}

impl<T: Routable + Debug + Clone> Router<T> {
    pub fn resolve_route(self) -> T {
        match self {
            Router::Route(route) => route,
            Router::Path(path_components) => T::route(path_components)
        }
    }
}

pub trait Routable {
    fn route(path_components: Vec<String>) -> Self;
}
