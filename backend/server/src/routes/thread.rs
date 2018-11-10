use auth_lib::user_authorization::{
    ModeratorUser,
    NormalUser,
};
use db::thread::{
    NewThread,
    Thread,
};
use error::*;
use identifiers::{
    forum::ForumUuid,
    thread::ThreadUuid,
    user::UserUuid,
};
use pool::Conn;
use rocket::Route;
use rocket_contrib::Json;
use routes::Routable;
use wire::thread::{
    MinimalThreadResponse,
    NewThreadRequest,
    ThreadResponse,
};

/// Creates a new thread with an Original Post (OP).
/// This operation is available to any logged in user.
#[post("/create", data = "<new_thread_request>")]
fn create_thread(
    new_thread_request: Json<NewThreadRequest>,
    user: NormalUser,
    conn: Conn,
) -> BackendResult<Json<ThreadResponse>> {
    let new_thread_request = new_thread_request.into_inner();
    if new_thread_request.author_uuid != user.user_uuid {
        return Err(Error::BadRequest);
    }

    let new_thread: NewThread = new_thread_request.clone().into();
    let post_content: String = new_thread_request.post_content;

    Thread::create_thread_with_initial_post(new_thread, post_content, &conn)
        .map(ThreadResponse::from)
        .map(Json)
}

/// This locks the thread, preventing further discussion.
/// This operation is available to moderators.
// TODO, consider creating a lock thread where the author of the thread can lock their own thread.
#[put("/lock/<thread_uuid>")]
fn lock_thread(
    thread_uuid: ThreadUuid,
    _moderator: ModeratorUser,
    conn: Conn,
) -> BackendResult<Json<MinimalThreadResponse>> {
    Thread::set_lock_status(thread_uuid, true, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

/// Unlocks a thread, allowing posting and editing again.
/// This operation is available to moderators.
#[put("/unlock/<thread_uuid>")]
fn unlock_thread(
    thread_uuid: ThreadUuid,
    _moderator: ModeratorUser,
    conn: Conn,
) -> BackendResult<Json<MinimalThreadResponse>> {
    Thread::set_lock_status(thread_uuid, false, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

/// Marks the thread as tombstoned, preventing it from showing up in requests and forbidding other operations on the thread.
/// This operation is available to moderators.
#[delete("/archive/<thread_uuid>")]
fn archive_thread(
    thread_uuid: ThreadUuid,
    _moderator: ModeratorUser,
    conn: Conn,
) -> BackendResult<Json<MinimalThreadResponse>> {
    Thread::archive_thread(thread_uuid, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

/// Gets the threads in the specified forum.
/// This operation is available to anyone.
#[get("/get/<forum_uuid>/<index>")]
fn get_threads_by_forum_id(
    forum_uuid: ForumUuid,
    index: i32,
    conn: Conn,
) -> BackendResult<Json<Vec<MinimalThreadResponse>>> {
    let results_per_page: i32 = 25;
    Thread::get_paginated(forum_uuid, index, results_per_page, &conn)
        .map_vec::<MinimalThreadResponse>()
        .map(Json)
}

/// Gets the entire contents of a thread.
/// The thread info, the posts, and the author of the thread.
///
/// If a JWT is attached, then the posts in the returned thread will have
/// info about that user's votes for each post returned.
#[get("/<thread_uuid>")]
fn get_thread_contents(
    thread_uuid: ThreadUuid,
    user: Option<NormalUser>,
    conn: Conn,
) -> BackendResult<Json<ThreadResponse>> {
    let user_uuid: Option<UserUuid> = user.map(|x| x.user_uuid);
    Thread::get_full_thread(thread_uuid, user_uuid, &conn)
        .map(ThreadResponse::from)
        .map(Json)
}

impl Routable for Thread {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_thread,
            lock_thread,
            unlock_thread,
            archive_thread,
            get_threads_by_forum_id,
            get_thread_contents,
        ]
    };
    const PATH: &'static str = "/thread/";
}
