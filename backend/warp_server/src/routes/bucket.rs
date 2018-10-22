use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
//use crate::db_integration::s.db.clone();
use db::Conn;
use db::bucket::Bucket;
use db::RetrievableUuid;
use crate::util::convert_and_json;
use wire::bucket::BucketResponse;
use crate::uuid_integration::uuid_wrap_filter;
use identifiers::bucket::BucketUuid;
use crate::state::State;
use pool::PooledConn;


// TODO This is incomplete because this section of the api will be rewritten to have a more minimal featureset
pub fn bucket_api(s: &State) -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching Bucket API");
    let api = get_bucket_by_uuid(s);

    warp::path("bucket")
        .and(api)
        .with(warp::log("bucket"))
        .boxed()
}


pub fn get_bucket_by_uuid(s: &State) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(uuid_wrap_filter())
        .and(s.db.clone())
        .and_then(|bucket_uuid: BucketUuid, conn: PooledConn| {
            Bucket::get_by_uuid(bucket_uuid.0, &conn)
                .map(convert_and_json::<Bucket, BucketResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}