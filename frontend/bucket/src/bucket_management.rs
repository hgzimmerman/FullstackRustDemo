use context::datatypes::bucket::BucketUsersData;


/// A component for approving and rejecting requests to join buckets.
pub struct BucketManagement {
    bucket_users:  Loadable<Vec<BucketUsersData>>
}

pub enum Msg {

}