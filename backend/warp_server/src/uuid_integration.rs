use warp;
use warp::filters::BoxedFilter;
use uuid::Uuid;
use warp::Filter;


/// TODO move this into util
pub fn uuid_filter() -> BoxedFilter<(Uuid,)> {
    warp::path::param()
        .boxed()
}

pub fn uuid_wrap_filter<T>() -> BoxedFilter<(T,)>
    where
        T: From<Uuid> + Send + 'static
{
    warp::path::param()
        .map(T::from)
        .boxed()
}