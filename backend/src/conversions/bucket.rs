use db::bucket::*;
use wire::bucket::*;


impl From<Bucket> for BucketResponse {
    fn from(bucket: Bucket) -> BucketResponse {
        BucketResponse {
            id: bucket.id,
            bucket_name: bucket.bucket_name,
            is_public: bucket.is_public,
        }
    }
}

impl From<NewBucketRequest> for NewBucket {
    fn from(new_bucket_request: NewBucketRequest) -> NewBucket {
        NewBucket {
            bucket_name: new_bucket_request.bucket_name,
            is_public: new_bucket_request.is_public,
        }
    }
}
