use schema::buckets;
use error::*;
use db::Conn;
use db::Retrievable;
use db::Creatable;
use db::Deletable;
use db::CRD;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::result::Error;
use diesel::ExpressionMethods;

#[derive(Debug, Clone, Identifiable, Queryable, Crd)]
#[insertable = "NewBucket"]
#[table_name = "buckets"]
pub struct Bucket {
    /// Primary Key.
    pub id: i32,
    /// The name of the bucket
    pub bucket_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "buckets"]
pub struct NewBucket {
    pub bucket_name: String,
}

impl Bucket {
    /// Gets a list of all buckets.
    pub fn get_buckets(conn: &Conn) -> Result<Vec<Bucket>, WeekendAtJoesError> {
        use schema::buckets::dsl::*;
        buckets
            .load::<Bucket>(conn.deref())
            .map_err(Bucket::handle_error)
    }
}

impl ErrorFormatter for Bucket {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError {
        handle_diesel_error(diesel_error, "Bucket")
    }
}
