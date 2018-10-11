use warp;
use warp::filters::BoxedFilter;
use uuid::Uuid;
use warp::Filter;

pub fn uuid_filter() -> BoxedFilter<(Uuid,)> {
    warp::path::param()
        .boxed()
}