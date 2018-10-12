use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
use crate::db_integration::db_filter;
use db::Conn;
use uuid::Uuid;
use wire::forum::ForumResponse;
use db::Forum;
use db::RetrievableUuid;
use crate::convert_and_json;
use crate::convert_vector_and_json;
use crate::uuid_integration::uuid_filter;
use crate::json_body_filter;
use crate::jwt::admin_user_filter;
use identifiers::user::UserUuid;
use wire::forum::NewForumRequest;
use db::CreatableUuid;

pub fn forum_api() -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Forum API");
    let api = get_forums()
        .or(get_forum())
        .or(create_forum())
        ;

    warp::path("forum")
        .and(api)
        .with(warp::log(""))
        .boxed()
}

/// Gets all the forums
fn get_forums() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(db_filter())
        .and_then(|conn: Conn|{
            Forum::get_all(&conn)
                .map(convert_vector_and_json::<Forum, ForumResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn get_forum() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(uuid_filter())
        .and(db_filter())
        .and_then(|uuid: Uuid, conn: Conn| {
            Forum::get_by_uuid(uuid, &conn)
                .map(convert_and_json::<Forum, ForumResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn create_forum() -> BoxedFilter<(impl Reply,)> {
    warp::post2()
        .and(json_body_filter(4))
        .and(admin_user_filter())
        .and(db_filter())
        .and_then(|request: NewForumRequest, _admin: UserUuid, conn: Conn|{
            Forum::create(request.into(), &conn)
                .map(convert_and_json::<Forum, ForumResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}