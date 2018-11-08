use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use error::Error;
//use crate::db_integration::s.db.clone();
//use db::Conn;
use db::bucket::Bucket;
use crate::util::convert_and_json;
use wire::bucket::BucketResponse;
use crate::uuid_integration::uuid_wrap_filter;
use identifiers::bucket::BucketUuid;
use crate::state::State;
use pool::PooledConn;
use crate::state::jwt::normal_user_filter;
use identifiers::user::UserUuid;
use crate::util::convert_vector_and_json;
use crate::logging::log_attach;
use crate::logging::HttpMethod;
use crate::util::json_body_filter;
use wire::bucket::NewBucketRequest;


pub fn bucket_api(s: &State) -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching Bucket API");
    let api = get_bucket_by_uuid(s)
        .or(create_bucket(s))
        .or(get_bucket_by_name(s))
        .or(get_buckets_belonging_to_user(s))
    ;

    warp::path("bucket")
        .and(api)
        .with(warp::log("bucket"))
        .boxed()
}


pub fn get_bucket_by_uuid(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "bucket/<uuid>");

    warp::get2()
        .and(uuid_wrap_filter())
        .and(s.db.clone())
        .and_then(|bucket_uuid: BucketUuid, conn: PooledConn| {
            Bucket::get_bucket(bucket_uuid, &conn)
                .map(convert_and_json::<Bucket, BucketResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn create_bucket(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "bucket/");

    warp::post2()
        .and(json_body_filter(4))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewBucketRequest, user_uuid: UserUuid, conn: PooledConn| {
            Bucket::create_bucket(request.into(), &conn)
                .map(convert_and_json::<Bucket, BucketResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn get_bucket_by_name(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "bucket/<name>");

    warp::get2()
        .and(warp::path::param())
        .and(s.db.clone())
        .and_then(|bucket_name: String, conn: PooledConn| {
            Bucket::get_bucket_by_name(bucket_name, &conn)
                .map(convert_and_json::<Bucket, BucketResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn get_buckets_belonging_to_user(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "bucket/owned");
    warp::get2()
        .and(warp::path::path("owned"))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: UserUuid, conn: PooledConn| {
            Bucket::get_buckets_user_owns(user_uuid, &conn)
                .map(convert_vector_and_json::<Bucket, BucketResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}
