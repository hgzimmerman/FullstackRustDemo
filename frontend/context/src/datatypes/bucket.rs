use wire::bucket::*;
use datatypes::user::UserData;
//use util::input::InputState;


#[derive(Clone, Debug, PartialEq, Default)]
pub struct BucketData {
    pub id: i32,
    pub bucket_name: String,
    pub is_public: bool
}

impl From<BucketResponse> for BucketData {
    fn from(response: BucketResponse) -> BucketData {
        BucketData {
            id: response.id,
            bucket_name: response.bucket_name,
            is_public: response.is_public
        }
    }
}

pub struct BucketUsersData {
    bucket: BucketData,
    users: Vec<UserData>
}
impl From<BucketUsersResponse> for BucketUsersData {
    fn from(response: BucketUsersResponse) -> BucketUsersData {
        BucketUsersData {
            bucket: BucketData::from(response.bucket),
            users: response.users.into_iter().map(UserData::from).collect()
        }
    }
}




