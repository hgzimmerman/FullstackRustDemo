use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;
use db::Retrievable;
use db::bucket::*;
use error::JoeResult;
use db::Conn;
use wire::bucket::*;
use auth::user_authorization::AdminUser;
use routes::convert_vector;
use db::Creatable;



/// Get all of the available buckets.
#[get("/buckets")]
fn get_buckets(conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {

    Bucket::get_all(&conn)
        .map(convert_vector)
        .map(Json)
}

/// Creates a new bucket.
/// The bucket represents a set of questions users can answer.
#[post("/create", data = "<new_bucket>")]
fn create_bucket(new_bucket: Json<NewBucketRequest>, _admin: AdminUser, conn: Conn) -> JoeResult<Json<BucketResponse>> {
    Bucket::create(new_bucket.into_inner().into(), &conn)
        .map(BucketResponse::from)
        .map(Json)
}



impl Routable for Bucket {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_buckets, create_bucket];
    const PATH: &'static str = "/bucket/";
}
