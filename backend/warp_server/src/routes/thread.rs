use crate::{
    state::{
        jwt::{
            moderator_user_filter,
            normal_user_filter,
            optional_normal_user_filter,
        },
        State,
    },
    util::{
        convert_and_json,
        convert_vector_and_json,
        json_body_filter,
    },
    uuid_integration::uuid_wrap_filter,
};
use db::thread::{
    MinimalThreadData,
    NewThread,
    Thread,
    ThreadData,
};
use error::Error;
use identifiers::{
    forum::ForumUuid,
    thread::ThreadUuid,
    user::UserUuid,
};
use pool::PooledConn;
use warp::{
    self,
    filters::BoxedFilter,
    reply::Reply,
    Filter,
};
use wire::thread::{
    MinimalThreadResponse,
    NewThreadRequest,
    ThreadResponse,
};

pub fn thread_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Thread API");
    let api = create_thread(s)
        .or(lock_thread(s))
        .or(unlock_thread(s))
        .or(archive_thread(s))
        .or(get_threads_by_forum_id(s))
        .or(get_thread_contents(s));

    warp::path("thread").and(api).with(warp::log("thread")).boxed()
}

pub fn create_thread(s: &State) -> BoxedFilter<(impl Reply,)> {
    warp::post2()
        .and(json_body_filter(20))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewThreadRequest, user_uuid: UserUuid, conn: PooledConn| {
            if request.author_uuid != user_uuid {
                return Error::BadRequest.reject();
            }

            let new_thread: NewThread = request.clone().into();
            let post_content: String = request.post_content;

            Thread::create_thread_with_initial_post(new_thread, post_content, &conn)
                .map(convert_and_json::<ThreadData, ThreadResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn lock_thread(s: &State) -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(warp::path("lock"))
        .and(uuid_wrap_filter())
        .and(moderator_user_filter(s))
        .and(s.db.clone())
        .and_then(|thread_uuid: ThreadUuid, _moderator: UserUuid, conn: PooledConn| {
            Thread::set_lock_status(thread_uuid, true, &conn)
                .map(convert_and_json::<MinimalThreadData, MinimalThreadResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn unlock_thread(s: &State) -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(warp::path("unlock"))
        .and(uuid_wrap_filter())
        .and(moderator_user_filter(s))
        .and(s.db.clone())
        .and_then(|thread_uuid: ThreadUuid, _moderator: UserUuid, conn: PooledConn| {
            Thread::set_lock_status(thread_uuid, false, &conn)
                .map(convert_and_json::<MinimalThreadData, MinimalThreadResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn archive_thread(s: &State) -> BoxedFilter<(impl Reply,)> {
    warp::delete2()
        .and(warp::path("archive"))
        .and(uuid_wrap_filter::<ThreadUuid>())
        .and(moderator_user_filter(s))
        .and(s.db.clone())
        .and_then(|thread_uuid: ThreadUuid, _moderator: UserUuid, conn: PooledConn| {
            Thread::archive_thread(thread_uuid, &conn)
                .map(convert_and_json::<MinimalThreadData, MinimalThreadResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn get_threads_by_forum_id(s: &State) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path("get")) // TODO: this api naming scheme is braindead
        .and(uuid_wrap_filter::<ForumUuid>())
        .and(warp::path::param::<i32>())
        .and(s.db.clone())
        .and_then(|forum_uuid: ForumUuid, index: i32, conn: PooledConn| {
            let results_per_page: i32 = 25;
            Thread::get_paginated(forum_uuid, index, results_per_page, &conn)
                .map(convert_vector_and_json::<MinimalThreadData, MinimalThreadResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

pub fn get_thread_contents(s: &State) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(uuid_wrap_filter::<ThreadUuid>())
        .and(optional_normal_user_filter(s))
        .and(s.db.clone())
        .and_then(
            |thread_uuid: ThreadUuid, user_uuid: Option<UserUuid>, conn: PooledConn| {
                Thread::get_full_thread(thread_uuid, user_uuid, &conn)
                    .map(convert_and_json::<ThreadData, ThreadResponse>)
                    .map_err(Error::simple_reject)
            },
        )
        .boxed()
}
