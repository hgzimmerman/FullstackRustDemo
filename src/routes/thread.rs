use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::thread::{Thread, NewThread};
use db::post::NewPost;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::thread::{NewThreadRequest, ThreadResponse};
use requests_and_responses::thread::MinimalThreadResponse;
use chrono::Utc;
use auth::user_authorization::NormalUser;
use auth::user_authorization::ModeratorUser;

use db::thread::{MinimalThreadData, ThreadData};


impl From<NewThreadRequest> for NewThread {
    fn from(request: NewThreadRequest) -> NewThread {
        NewThread {
            forum_id: request.forum_id,
            author_id: request.author_id,
            created_date: Utc::now().naive_utc(),
            locked: false,
            archived: false,
            title: request.title,
        }
    }
}

impl From<NewThreadRequest> for NewPost {
    fn from(request: NewThreadRequest) -> NewPost {
        // Just grab the post field from the thread request.
        NewPost::from(request.post)
    }
}



impl From<ThreadData> for ThreadResponse {
    fn from(data: ThreadData) -> ThreadResponse {
        ThreadResponse {
            id: data.thread.id,
            title: data.thread.title,
            author: data.user.into(),
            posts: data.post.into(),
            created_date: data.thread.created_date,
            locked: data.thread.locked,
        }
    }
}



impl From<MinimalThreadData> for MinimalThreadResponse {
    fn from(data: MinimalThreadData) -> MinimalThreadResponse {
        MinimalThreadResponse {
            id: data.thread.id,
            title: data.thread.title,
            author: data.user.into(),
            created_date: data.thread.created_date,
            locked: data.thread.locked,
        }
    }
}


#[post("/create", data = "<new_thread_request>")]
fn create_thread(new_thread_request: Json<NewThreadRequest>, _normal_user: NormalUser, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let new_thread_request = new_thread_request.into_inner();

    let new_thread: NewThread = new_thread_request.clone().into();
    let new_original_post: NewPost = new_thread_request.into();

    Thread::create_thread_with_initial_post(new_thread, new_original_post, &conn)
        .map(ThreadResponse::from)
        .map(Json)
}

#[put("/lock/<thread_id>")]
fn lock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::set_lock_status(thread_id, true, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

#[put("/unlock/<thread_id>")]
fn unlock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::set_lock_status(thread_id, false, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

#[delete("/archive/<thread_id>")]
fn archive_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<MinimalThreadResponse>, WeekendAtJoesError> {
    Thread::archive_thread(thread_id, &conn)
        .map(MinimalThreadResponse::from)
        .map(Json)
}

#[get("/get/<forum_id>")]
fn get_threads_by_forum_id(forum_id: i32, conn: Conn) -> Result<Json<Vec<MinimalThreadResponse>>, WeekendAtJoesError> {
    // TODO move the 25 into a parameter
    // TODO make this more efficient by doing a join in the database method
    Thread::get_threads_in_forum(forum_id, 25, &conn)
        .map(|threads| {
            threads
                .into_iter()
                .map(MinimalThreadResponse::from)
                .collect()
        })
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
