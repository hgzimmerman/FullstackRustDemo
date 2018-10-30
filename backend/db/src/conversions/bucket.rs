use crate::bucket::*;
use wire::bucket::*;
use wire::user::*;
use identifiers::bucket::BucketUuid;

use chrono::{NaiveDateTime, Utc, Duration};

impl From<Bucket> for BucketResponse {
    fn from(bucket: Bucket) -> BucketResponse {
        let is_public: bool = if let Some(until) = bucket.is_public_until {
            Utc::now().naive_utc() < until // If the current time is less than the expiry time on publicity, then the bucket is public.
        } else {
            false
        };

        BucketResponse {
            uuid: BucketUuid(bucket.uuid),
            bucket_name: bucket.bucket_name,
            is_public,
        }
    }
}

impl From<NewBucketRequest> for NewBucket {
    fn from(new_bucket_request: NewBucketRequest) -> NewBucket {
        let is_public_until: Option<NaiveDateTime> = if new_bucket_request.is_public {
            Some(Utc::now().naive_utc() + Duration::days(1))
        } else {
            None
        };

        NewBucket {
            bucket_name: new_bucket_request.bucket_name,
            is_public_until,
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
