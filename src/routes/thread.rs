use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::thread::{Thread, NewThread};
use db::post::NewPost;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::thread::{NewThreadRequest, ThreadResponse};
use requests_and_responses::thread::MinimalThreadResponse;
use auth::user_authorization::NormalUser;
use auth::user_authorization::ModeratorUser;
use error::VectorMappable;


/// Creates a new thread with an Original Post (OP).
/// This operation is available to any logged in user.
#[post("/create", data = "<new_thread_request>")]
fn create_thread(new_thread_request: Json<NewThreadRequest>, _normal_user: NormalUser, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let new_thread_request = new_thread_request.into_inner();

    let new_thread: NewThread = new_thread_request.clone().into();
    let new_original_post: NewPost = new_thread_request.into();

    Thread::create_thread_with_initial_post(new_thread, new_original_post, &conn)
        .map(ThreadResponse::from)
        .map(Json)
}

/// This locks the thread, preventing further discussion.
/// This operation is available to moderators.
// TODO, consider creating an alternative lock thread where the author of the thread can lock their own thread.
#[put("/lock/<thread_id>")]
fn lock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::set_lock_status(thread_id, true, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

/// Unlocks a thread, allowing posting and editing again.
/// This operation is available to moderators.
#[put("/unlock/<thread_id>")]
fn unlock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::set_lock_status(thread_id, false, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

/// Marks the thread as tombstoned, preventing it from showing up in requests and forbidding other operations on the thread.
/// This operation is available to moderators.
#[delete("/archive/<thread_id>")]
fn archive_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::archive_thread(thread_id, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

/// Gets the threads in the specified forum.
/// This operation is available to anyone.
#[get("/get/<forum_id>")]
fn get_threads_by_forum_id(forum_id: i32, conn: Conn) -> Result<Json<Vec<MinimalThreadResponse>>, WeekendAtJoesError> {
    // TODO move the 25 into a parameter
    // TODO make this more efficient by doing a join in the database method
    Thread::get_threads_in_forum(forum_id, 25, &conn)
        .map_vec::<MinimalThreadResponse>()
        // .map(|threads| {
        //     threads
        //         .into_iter()
        //         .map(MinimalThreadResponse::from)
        //         .collect()
        // })
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
        ]
    };
    const PATH: &'static str = "/thread/";
}
