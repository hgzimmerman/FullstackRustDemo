use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;
use db::RetrievableUuid;
use db::bucket::*;
use error::JoeResult;
use db::Conn;
use wire::bucket::*;
use wire::user::UserResponse;
use auth::user_authorization::NormalUser;
use routes::convert_vector;
use db::CreatableUuid;
use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;
use uuid::Uuid;

use error::*;

#[derive(FromForm)]
struct PublicParam {
    is_public: bool,
}

/// Get all of the available buckets.
#[get("/public")]
fn get_public_buckets(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {
    Bucket::get_public_buckets(user.user_uuid, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Gets the buckets the user is approved to join.
#[get("/approved")]
fn get_approved_buckets_for_user(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketResponse>>> {
    Bucket::get_buckets_user_belongs_to(user.user_uuid, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Approves the user, allowing them to join the bucket session.
/// Only a one way set transformation.
/// You need to remove the user from the bucket.
/// The user being approved already needs to have registered with the bucket in question.
#[put("/<bucket_uuid>/approval?<user_uuid>")]
fn approve_user_for_bucket(bucket_uuid: BucketUuid, user_uuid: UserUuid, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if !Bucket::is_user_owner(user.user_uuid, bucket_uuid, &conn)? {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User must be an owner of the bucket in order to approve users." };
        return Err(e);
    }

    Bucket::set_user_approval(user_uuid, bucket_uuid, true, &conn)
}

/// Entirely removes the user from the bucket.
#[delete("/<bucket_uuid>?<user_uuid>")]
fn remove_user_from_bucket(bucket_uuid: BucketUuid, user_uuid: UserUuid, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if !Bucket::is_user_owner(user.user_uuid, bucket_uuid, &conn)? {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User must be an owner of the bucket in order to approve users." };
        return Err(e);
    }
    Bucket::remove_user_from_bucket(user_uuid, bucket_uuid, &conn)
}

/// Allows the owners of buckets to set the is_public flag for their buckets
/// This will prevent other buckets from
#[put("/<bucket_uuid>/publicity?<is_public_param>")]
fn set_publicity(bucket_uuid: BucketUuid, is_public_param: PublicParam, user: NormalUser, conn: Conn) -> JoeResult<()> {
    if !Bucket::is_user_owner(user.user_uuid, bucket_uuid, &conn)? {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User must be an owner of the bucket in order to approve users." };
        return Err(e);
    }
    Bucket::set_bucket_publicity(bucket_uuid, is_public_param.is_public, &conn)
}

/// Gets the bucket at the given Id.
/// Requires that the user requesting the bucket is approved to join.
#[get("/<bucket_uuid>")]
fn get_bucket(bucket_uuid: BucketUuid, user: NormalUser, conn: Conn) -> JoeResult<Json<BucketResponse>> {
    // If the user isn't approved then return a 403.
    if !Bucket::is_user_approved(user.user_uuid, bucket_uuid, &conn) {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User has not been approved to participate in the bucket questions session." };
        return Err(e);
    }

    Bucket::get_by_uuid(bucket_uuid.0, &conn)
        .map(BucketResponse::from)
        .map(Json)
}


/// For the buckets that the active user owns, return the list of users that have requested to join the bucket,
/// but require the active user to approve their request.
#[get("/unapproved_users_for_owned_buckets")]
fn get_unapproved_users_in_buckets_owned_by_user(user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<BucketUsersResponse>>> {
    Bucket::get_users_requiring_approval_for_owned_buckets(user.user_uuid, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Gets all of the users in the bucket, excluding the user that made the request
#[get("/<bucket_uuid>/users")]
fn get_users_in_bucket(bucket_uuid: BucketUuid, user: NormalUser, conn: Conn) -> JoeResult<Json<Vec<UserResponse>>> {
    if !Bucket::is_user_approved(user.user_uuid, bucket_uuid, &conn) {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User has not been approved to participate in the bucket questions session." };
        return Err(e);
    }
    use db::user::User;

    let users = Bucket::get_users_with_approval(bucket_uuid, &conn)?
        .into_iter()
        .filter( |u: &User| u.uuid != user.user_uuid.0 ) // Filter out the user making the request.
        .map(UserResponse::from)
        .collect();
    Ok(Json(users))
}

/// Given a bucket id, determine if the current user owns the bucket.
#[get("/<bucket_uuid>/user_owner_status")]
fn get_is_current_user_owner(bucket_uuid: BucketUuid, user: NormalUser, conn: Conn) -> JoeResult<Json<bool>> {
    if !Bucket::is_user_approved(user.user_uuid, bucket_uuid, &conn) {
        let e = WeekendAtJoesError::NotAuthorized { reason: "User has not been approved to participate in the bucket questions session." };
        return Err(e);
    }

    Bucket::is_user_owner(user.user_uuid, bucket_uuid, &conn)
        .map(Json)
}

/// Make a request to join the bucket.
/// The user will not immediately be able to join the bucket, but after the owner of the bucket approves them
/// they will gain access to internal bucket information.
#[post("/<bucket_uuid>/user_join_request")]
fn request_to_join_bucket(bucket_uuid: BucketUuid, user: NormalUser, conn: Conn) -> JoeResult<()> {
    let bucket_uuid: Uuid = bucket_uuid.0;
    let user_uuid: Uuid = user.user_uuid.0;
    let new_bucket_user = NewBucketUser {
        bucket_uuid,
        user_uuid,
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
        bucket_uuid: bucket_response.uuid.0,
        user_uuid: user.user_uuid.0,
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
