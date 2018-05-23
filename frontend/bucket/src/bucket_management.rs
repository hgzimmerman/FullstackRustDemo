use context::datatypes::bucket::*;


pub struct BucketManagement {
    users:  Loadable<BucketUsersData>
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: Option<BucketData>
}

pub enum Msg {

}