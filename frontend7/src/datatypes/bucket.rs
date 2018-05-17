use wire::bucket::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BucketData {
    pub id: i32,
    pub bucket_name: String
}

impl From<BucketResponse> for BucketData {
    fn from(response: BucketResponse) -> BucketData {
        BucketData {
            id: response.id,
            bucket_name: response.bucket_name
        }
    }
}