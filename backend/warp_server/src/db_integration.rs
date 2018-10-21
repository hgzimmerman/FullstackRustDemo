use warp::Filter;
//use warp::reject::Rejection;
use warp::filters::BoxedFilter;
use crate::error::Error;

use db::Conn;
use db::init_pool;
use db::Pool;
use db::DATABASE_URL;

//pub fn db_filter_impl() -> impl Filter<Extract = (Conn,), Error = Rejection>  {
//    warp::any()
//        .and_then(move || get_conn(&POOL).map_err(|_| warp::reject().with(Error::DatabaseUnavailable)) )
//}

pub fn db_filter() -> BoxedFilter<(Conn,)> {
    warp::any()
        .and_then(move || get_conn(&POOL).map_err(|_| warp::reject().with(Error::DatabaseUnavailable)) )
        .boxed()
}



fn get_conn(pool: &Pool) -> Result<Conn, ()> {
    match pool.get() {
        Ok(conn) => Ok(Conn::new(conn)),
        Err(_) => Err(()) // TODO this should be a timeout error, because the pool.get() internally waits for a timeout.// TODO this should represent a SERVICE_UNAVAILABLE or possibly wait for a conn to free until a timeout occurs
    }
}

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref POOL: Pool = init_pool(DATABASE_URL);
}
