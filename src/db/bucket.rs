use schema::buckets;
use error::WeekendAtJoesError;
use db::Conn;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use db::handle_diesel_error;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[table_name="buckets"]
pub struct Bucket {
    /// Primary Key.
    pub id: i32,
    /// The name of the bucket
    pub bucket_name: String
}

#[derive(Insertable, Debug)]
#[table_name="buckets"]
pub struct NewBucket {
    pub bucket_name: String
}

impl Bucket {
    /// Creates a new bucket
    pub fn create_bucket(new_bucket: NewBucket, conn: &Conn) -> Result<Bucket, WeekendAtJoesError> {
        use schema::buckets;

        diesel::insert_into(buckets::table)
            .values(&new_bucket)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Bucket"))
    }

    /// Gets a list of all buckets.
    pub fn get_buckets(conn: &Conn) -> Result<Vec<Bucket>, WeekendAtJoesError> {
        use schema::buckets::dsl::*;
        buckets
            .load::<Bucket>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Bucket")) 
    }

    /// Gets a bucket by id.
    pub fn get_bucket(bucket_id: i32, conn: &Conn) -> Result<Bucket, WeekendAtJoesError> {
        use schema::buckets::dsl::*;

        // Gets the first bucket that matches the id.
        buckets 
            .find(bucket_id)
            .first::<Bucket>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Forum"))

    }
}