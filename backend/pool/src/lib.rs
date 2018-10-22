extern crate r2d2;
extern crate diesel;
extern crate r2d2_diesel;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

/// Holds a bunch of connections to the database and hands them out to routes as needed.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

/// Initializes the pool.
pub fn init_pool(db_url: &str) -> Pool {
    //    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect(
        "db pool",
    )
}
