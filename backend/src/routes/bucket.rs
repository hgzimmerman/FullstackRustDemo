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

use error::*;

use rocket::request::FromForm;

#[derive(FromForm)]
struct UserIdParam {
    user_id: i32
}

#[derive(FromForm)]
struct PublicParam {
    is_public: bool
}

/// Get all of the available buckets.
#[get("/public")]
fn get_public_buckets(_user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {
    Bucket::get_public_buckets(&conn)
        .map(convert_vector)
        .map(Json)
}

#[get("/approved")]
fn get_approved_buckets_for_user(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {
    Bucket::get_buckets_user_belongs_to(user.user_id, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Only a one way set transformation.
/// You need to remove the user from the bucket.
/// The user being approved already needs to have registered with the bucket in question.
#[put("/<bucket_id>/approval?<user_id_param>")]
fn approve_user_for_bucket(bucket_id: i32, user_id_param: UserIdParam, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if ! Bucket::is_user_owner(user.user_id, &conn) {
        let e = WeekendAtJoesError::NotAuthorized{reason: "User must be an owner of the bucket in order to approve users."};
        return Err(e)
    }

    Bucket::set_user_approval( user_id_param.user_id, bucket_id, true, &conn)
}


#[delete("/<bucket_id>?<user_id_param>")]
fn remove_user_from_bucket(bucket_id: i32, user_id_param: UserIdParam, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if ! Bucket::is_user_owner(user.user_id, &conn) {
        let e = WeekendAtJoesError::NotAuthorized{reason: "User must be an owner of the bucket in order to approve users."};
        return Err(e)
    }
    Bucket::remove_user_from_bucket(user_id_param.user_id, bucket_id, &conn)
}

/// Allows the owners of buckets to set the is_public flag for their buckets
/// This will prevent other buckets from
#[put("/<bucket_id>/publicity?<is_public_param>")]
fn set_publicity(bucket_id: i32, is_public_param: PublicParam, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if ! Bucket::is_user_owner(user.user_id, &conn) {
        let e = WeekendAtJoesError::NotAuthorized{reason: "User must be an owner of the bucket in order to approve users."};
        return Err(e)
    }
    Bucket::set_bucket_publicity(bucket_id, is_public_param.is_public, &conn)
}

/// Gets the bucket at the given Id.
/// Requires that the user requesting the bucket is approved to join.
#[get("/<bucket_id>")]
fn get_bucket(bucket_id: i32, user: NormalUser, conn: Conn) -> JoeResult<Json<BucketResponse>> {
    // If the user isn't approved then return a 403.
    if ! Bucket::is_user_approved(user.user_id, &conn) {
        let e = WeekendAtJoesError::NotAuthorized{reason: "User has not been approved to participate in the bucket questions session."};
        return Err(e)
    }

    Bucket::get_by_id(bucket_id, &conn)
        .map(BucketResponse::from)
        .map(Json)
}

/// Creates a new bucket.
/// The bucket represents a set of questions users can answer.
#[post("/create", data = "<new_bucket>")]
fn create_bucket(new_bucket: Json<NewBucketRequest>, user: NormalUser, conn: Conn) -> JoeResult<Json<BucketResponse>> {
    // create the bucket
    let bucket_response = Bucket::create(new_bucket.into_inner().into(), &conn)
        .map(BucketResponse::from)
        .map(Json)?;

    // Add the user who made the request to the bucket as the owner
    let new_bucket_user = NewBucketUser {
        bucket_id: bucket_response.id,
        user_id: user.user_id,
        owner: true,
        approved: true
    };
    Bucket::add_user_to_bucket(new_bucket_user, &conn)?;

    Ok(bucket_response)
}




impl Routable for Bucket {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![
        get_public_buckets,
        get_approved_buckets_for_user,
        set_publicity,
        approve_user_for_bucket,
        remove_user_from_bucket,
        get_bucket,
        create_bucket
    ];
    const PATH: &'static str = "/buckets/";
}
