use schema::buckets;
use db::Conn;
use std::ops::Deref;


#[derive(Debug, Clone, Identifiable, Queryable, Crd, ErrorHandler)]
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
