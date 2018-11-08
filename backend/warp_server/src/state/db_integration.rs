use warp::{
    Filter,
    filters::BoxedFilter
};
use crate::error::Error;
use pool::{
    Pool,
    PooledConn
};

/// With access to a pool, the filter will be able to get a pooled connection that is then used to make calls to the db.
/// Since Pool is actually an Arc around the shared resource pool, cloning it merely copies the pointer, meaning that the
/// pool still has a finite number of connections and the expected limiting of connections is maintained.
pub fn db_filter(pool: Pool) -> BoxedFilter<(PooledConn,)> {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool_2: Pool| pool_2.get().map_err(|_| Error::DatabaseUnavailable.simple_reject()) )
        .boxed()
}



