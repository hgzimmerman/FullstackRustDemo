use db::bucket::*;
use wire::bucket::*;
use wire::user::*;

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

impl From<UsersInBucketData> for BucketUsersResponse {
    fn from(data: UsersInBucketData) -> BucketUsersResponse {
        BucketUsersResponse {
            bucket: BucketResponse::from(data.bucket),
            users: data.users
                .into_iter()
                .map(UserResponse::from)
                .collect(),
        }
    }
}
