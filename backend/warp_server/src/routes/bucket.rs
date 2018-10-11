use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
use crate::uuid_integration::uuid_filter;
use crate::db_integration::db_filter;
use db::Conn;
use uuid::Uuid;
use db::bucket::Bucket;
use db::RetrievableUuid;
use crate::convert_and_json;
use wire::bucket::BucketResponse;


// TODO This is incomplete because this section of the api will be rewritten to have a more minimal featureset
pub fn bucket_api() -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching Bucket API");
    let api = get_bucket_by_uuid();

    warp::path("bucket")
        .and(api)
        .with(warp::log("bucket"))
        .boxed()
}


pub fn get_bucket_by_uuid() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(uuid_filter())
        .and(db_filter())
        .and_then(|uuid: Uuid, conn: Conn| {
            Bucket::get_by_uuid(uuid, &conn)
                .map(convert_and_json::<Bucket, BucketResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}