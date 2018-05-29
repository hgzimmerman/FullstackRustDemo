use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;
use db::Retrievable;
use db::bucket::*;
use error::JoeResult;
use db::Conn;
use wire::bucket::*;
use wire::user::UserResponse;
use auth::user_authorization::AdminUser;
use auth::user_authorization::NormalUser;
use routes::convert_vector;
use db::Creatable;

use error::*;

use rocket::request::FromForm;

#[derive(FromForm)]
struct UserIdParam {
    user_id: i32,
}

#[derive(FromForm)]
struct PublicParam {
    is_public: bool,
}

/// Get all of the available buckets.
#[get("/public")]
fn get_public_buckets(_user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {
    Bucket::get_public_buckets(&conn)
        .map(convert_vector)
        .map(Json)
}

/// Gets the buckets the user is approved to join.
#[get("/approved")]
fn get_approved_buckets_for_user(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {
    Bucket::get_buckets_user_belongs_to(user.user_id, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Approves the user, allowing them to join the bucket session.
/// Only a one way set transformation.
/// You need to remove the user from the bucket.
/// The user being approved already needs to have registered with the bucket in question.
#[put("/<bucket_id>/approval?<user_id_param>")]
fn approve_user_for_bucket(bucket_id: i32, user_id_param: UserIdParam, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if !Bucket::is_user_owner(user.user_id, bucket_id, &conn)? {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User must be an owner of the bucket in order to approve users." };
        return Err(e);
    }

    Bucket::set_user_approval(user_id_param.user_id, bucket_id, true, &conn)
}

/// Entirely removes the user from the bucket.
#[delete("/<bucket_id>?<user_id_param>")]
fn remove_user_from_bucket(bucket_id: i32, user_id_param: UserIdParam, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if !Bucket::is_user_owner(user.user_id, bucket_id, &conn)? {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User must be an owner of the bucket in order to approve users." };
        return Err(e);
    }
    Bucket::remove_user_from_bucket(user_id_param.user_id, bucket_id, &conn)
}

/// Allows the owners of buckets to set the is_public flag for their buckets
/// This will prevent other buckets from
#[put("/<bucket_id>/publicity?<is_public_param>")]
fn set_publicity(bucket_id: i32, is_public_param: PublicParam, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if !Bucket::is_user_owner(user.user_id, bucket_id, &conn)? {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User must be an owner of the bucket in order to approve users." };
        return Err(e);
    }
    Bucket::set_bucket_publicity(bucket_id, is_public_param.is_public, &conn)
}

/// Gets the bucket at the given Id.
/// Requires that the user requesting the bucket is approved to join.
#[get("/<bucket_id>")]
fn get_bucket(bucket_id: i32, user: NormalUser, conn: Conn) -> JoeResult<Json<BucketResponse>> {
    // If the user isn't approved then return a 403.
    if !Bucket::is_user_approved(user.user_id, bucket_id, &conn) {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User has not been approved to participate in the bucket questions session." };
        return Err(e);
    }

    Bucket::get_by_id(bucket_id, &conn)
        .map(BucketResponse::from)
        .map(Json)
}


#[get("/unapproved_users_for_owned_buckets")]
fn get_unapproved_users_in_buckets_owned_by_user(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketUsersResponse>>> {
    Bucket::get_users_requiring_approval_for_owned_buckets(user.user_id, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Gets all of the users in the bucket, excluding the user that made the request
///
#[get("/<bucket_id>/users")]
fn get_users_in_bucket(bucket_id: i32, user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<UserResponse>>> {
    if !Bucket::is_user_approved(user.user_id, bucket_id, &conn) {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User has not been approved to participate in the bucket questions session." };
        return Err(e);
    }

    let users = Bucket::get_users_with_approval(bucket_id, &conn)?
        .into_iter()
        .filter( |m_user| m_user.id != user.user_id ) // Filter out the user making the request.
        .map(UserResponse::from)
        .collect();
    Ok(Json(users))
}

#[get("/<bucket_id>/user_owner_status")]
fn get_is_current_user_owner(bucket_id: i32, user: NormalUser, conn: Conn) -> JoeResult<Json<bool>> {
    if !Bucket::is_user_approved(user.user_id, bucket_id, &conn) {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User has not been approved to participate in the bucket questions session." };
        return Err(e);
    }

    Bucket::is_user_owner(user.user_id, bucket_id, &conn)
        .map(Json)
}

#[post("/<bucket_id>/user_join_request")]
fn request_to_join_bucket(bucket_id: i32, user: NormalUser, conn: Conn) -> JoeResult<()> {
    let new_bucket_user = NewBucketUser {
        bucket_id,
        user_id: user.user_id,
        owner: false,
        approved: false,
    };

    Bucket::add_user_to_bucket(new_bucket_user, &conn)
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
        approved: true,
    };
    Bucket::add_user_to_bucket(new_bucket_user, &conn)?;

    Ok(bucket_response)
}




impl Routable for Bucket {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            get_public_buckets,
            get_approved_buckets_for_user,
            set_publicity,
            approve_user_for_bucket,
            remove_user_from_bucket,
            get_bucket,
            create_bucket,
            get_users_in_bucket,
            get_unapproved_users_in_buckets_owned_by_user,
            get_is_current_user_owner,
            request_to_join_bucket,
        ]
    };
    const PATH: &'static str = "/buckets/";
}
