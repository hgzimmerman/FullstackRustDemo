use crate::{
    logging::{
        log_attach,
        HttpMethod,
    },
    state::{
        jwt::admin_user_filter,
        State,
    },
    util::{
        convert_and_json,
        convert_vector_and_json,
        json_body_filter,
    },
    uuid_integration::uuid_wrap_filter,
};
use db::Forum;
use error::Error;
use identifiers::{
    forum::ForumUuid,
    user::UserUuid,
};
use pool::PooledConn;
use warp::{
    filters::BoxedFilter,
    reply::Reply,
    Filter,
};
use wire::forum::{
    ForumResponse,
    NewForumRequest,
};

pub fn forum_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Forum API");
    let api = get_forums(s).or(get_forum(s)).or(create_forum(s));

    warp::path("forum").and(api).with(warp::log("")).boxed()
}

/// Gets all the forums
fn get_forums(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "forum/");

    warp::get2()
        .and(s.db.clone())
        .and_then(|conn: PooledConn| {
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
        .and_then(|request: NewForumRequest, _admin: UserUuid, conn: PooledConn| {
            Forum::create_forum(request.into(), &conn)
                .map(convert_and_json::<Forum, ForumResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}
