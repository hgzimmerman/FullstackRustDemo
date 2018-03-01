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

#[derive(Debug, Clone, Identifiable, Queryable)]
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

impl Creatable<NewBucket> for Bucket {
    fn create(new_bucket: NewBucket, conn: &Conn) -> Result<Bucket, WeekendAtJoesError> {
        use schema::buckets;

        diesel::insert_into(buckets::table)
            .values(&new_bucket)
            .get_result(conn.deref())
            .map_err(Bucket::handle_error)
    }
}

impl<'a> Retrievable<'a> for Bucket {
    /// Gets a bucket by id.
    fn get_by_id(bucket_id: i32, conn: &Conn) -> Result<Bucket, WeekendAtJoesError> {
        use schema::buckets::dsl::*;

        // Gets the first bucket that matches the id.
        buckets
            .find(bucket_id)
            .first::<Bucket>(conn.deref())
            .map_err(Bucket::handle_error)
    }
}

impl<'a> Deletable<'a> for Bucket {
    fn delete_by_id(bucket_id: i32, conn: &Conn) -> Result<Bucket, WeekendAtJoesError> {
        use schema::buckets::dsl::*;

        diesel::delete(buckets.filter(id.eq(bucket_id)))
            .get_result(conn.deref())
            .map_err(Bucket::handle_error)
    }
}

impl<'a> CRD<'a, NewBucket> for Bucket {}

impl ErrorFormatter for Bucket {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError {
        handle_diesel_error(diesel_error, "Bucket")
    }
}
