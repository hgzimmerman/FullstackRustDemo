use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::bucket::*;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::bucket::*;
use auth::user_authorization::AdminUser;
use routes::convert_vector;

impl From<Bucket> for BucketResponse {
    fn from(bucket: Bucket) -> BucketResponse {
        BucketResponse {
            id: bucket.id,
            bucket_name: bucket.bucket_name
        }
    }
}

impl From<NewBucketRequest> for NewBucket {
    fn from(new_bucket_request: NewBucketRequest) -> NewBucket {
        NewBucket {
            bucket_name: new_bucket_request.bucket_name
        }
    }
}

#[get("/buckets")]
fn get_buckets(conn: Conn) -> Result<Json<Vec<BucketResponse>>, WeekendAtJoesError> {

    Bucket::get_buckets(&conn)
        .map(convert_vector)
        .map(Json)
}

#[post("/create", data = "<new_bucket>")]
fn create_bucket(new_bucket: Json<NewBucketRequest>, _admin: AdminUser, conn: Conn) -> Result<Json<BucketResponse>, WeekendAtJoesError> {
    Bucket::create_bucket(new_bucket.into_inner().into(), &conn)
        .map(BucketResponse::from)
        .map(Json)
}



impl Routable for Bucket {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_buckets, create_bucket];
    const PATH: &'static str = "/bucket/";
}