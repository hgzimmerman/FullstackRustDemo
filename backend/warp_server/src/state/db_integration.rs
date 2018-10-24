use warp::Filter;
//use warp::reject::Rejection;
use warp::filters::BoxedFilter;
use crate::error::Error;
//use r2d2;

//use db::Conn;
//use db::init_pool;
//use db::Pool;

use pool::Pool;
use pool::PooledConn;
//use pool::init_pool;
//use db::DATABASE_URL;

//#[cfg(test)]
//use testing_common::setup::DATABASE_URL as TESTING_DATABASE_URL;


pub fn db_filter(pool: Pool) -> BoxedFilter<(PooledConn,)> {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool_2: Pool| pool_2.get().map_err(|_| Error::DatabaseUnavailable.simple_reject()) )
        .boxed()
}

