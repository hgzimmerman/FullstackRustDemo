use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;
use db::Retrievable;
use db::bucket::*;
use error::JoeResult;
use db::Conn;
use wire::bucket::*;
use auth::user_authorization::AdminUser;
use auth::user_authorization::NormalUser;
use routes::convert_vector;
use db::Creatable;



/// Get all of the available buckets.
#[get("/")]
fn get_buckets(conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {
    Bucket::get_all(&conn)
        .map(convert_vector)
        .map(Json)
}

/// gets the bucket at the given Id.
#[get("/<bucket_id>")]
fn get_bucket(bucket_id: i32, conn: Conn) -> JoeResult<Json<BucketResponse>> {
    Bucket::get_by_id(bucket_id, &conn)
        .map(BucketResponse::from)
        .map(Json)
}

/// Creates a new bucket.
/// The bucket represents a set of questions users can answer.
#[post("/create", data = "<new_bucket>")]
fn create_bucket(new_bucket: Json<NewBucketRequest>, _admin: NormalUser, conn: Conn) -> JoeResult<Json<BucketResponse>> {
    Bucket::create(new_bucket.into_inner().into(), &conn)
        .map(BucketResponse::from)
        .map(Json)
}



impl Routable for Bucket {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_buckets, get_bucket, create_bucket];
    const PATH: &'static str = "/buckets/";
}
