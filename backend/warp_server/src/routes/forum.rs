use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use error::Error;
//use crate::db_integration::s.db.clone();
//use db::Conn;
use wire::forum::ForumResponse;
use db::Forum;
use crate::util::convert_and_json;
use crate::util::convert_vector_and_json;
use crate::util::json_body_filter;
use crate::state::jwt::admin_user_filter;
use identifiers::user::UserUuid;
use wire::forum::NewForumRequest;

use crate::logging::log_attach;
use crate::logging::HttpMethod;
use crate::uuid_integration::uuid_wrap_filter;
use identifiers::forum::ForumUuid;
use crate::state::State;
use pool::PooledConn;

pub fn forum_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Forum API");
    let api = get_forums(s)
        .or(get_forum(s))
        .or(create_forum(s))
        ;

    warp::path("forum")
        .and(api)
        .with(warp::log(""))
        .boxed()
}

/// Gets all the forums
fn get_forums(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "forum/");

    warp::get2()
        .and(s.db.clone())
        .and_then(|conn: PooledConn|{
            Forum::get_forums(&conn)
                .map(convert_vector_and_json::<Forum, ForumResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn get_forum(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "forum/<uuid>");

    warp::get2()
        .and(uuid_wrap_filter())
        .and(s.db.clone())
        .and_then(|uuid: ForumUuid, conn: PooledConn| {
            Forum::get_forum(uuid, &conn)
                .map(convert_and_json::<Forum, ForumResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn create_forum(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "forum/");

    warp::post2()
        .and(json_body_filter(4))
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewForumRequest, _admin: UserUuid, conn: PooledConn|{
            Forum::create_forum(request.into(), &conn)
                .map(convert_and_json::<Forum, ForumResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}