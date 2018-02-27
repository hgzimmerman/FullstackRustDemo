use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::thread::{Thread, NewThread};
use db::post::{Post, NewPost};
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::thread::{NewThreadRequest, ThreadResponse};
use requests_and_responses::thread::MinimalThreadResponse;
use requests_and_responses::post::PostResponse;
use chrono::Utc;
use auth::user_authorization::NormalUser;
use auth::user_authorization::ModeratorUser;


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


impl Thread {
    /// The response requires both a post and a user to be attached.
    fn into_one_post_thread_response(self, post: Post, user: User) -> ThreadResponse {
        ThreadResponse {
            id: self.id,
            title: self.title,
            author: user.clone().into(),
            posts: post.into_childless_response(user),
            created_date: self.created_date,
            locked: self.locked,
        }
    }

    fn into_full_thread_response(self, conn: &Conn) -> Result<ThreadResponse, WeekendAtJoesError> {
        let post: Post = Post::get_root_post(self.id, conn)?;
        let post_response: PostResponse = post.into_post_response(conn)?;
        Ok(ThreadResponse {
            id: self.id,
            title: self.title,
            author: post_response.author.clone(),
            posts: post_response,
            created_date: self.created_date,
            locked: self.locked,
        })
    }

    fn into_minimal_thread_response(self, user: User) -> MinimalThreadResponse {
        MinimalThreadResponse {
            id: self.id,
            title: self.title,
            author: user.clone().into(),
            created_date: self.created_date,
            locked: self.locked,
        }
    }
}


#[post("/create", data = "<new_thread_request>")]
fn create_thread(new_thread_request: Json<NewThreadRequest>, _normal_user: NormalUser, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let new_thread_request = new_thread_request.into_inner();

    let new_thread: NewThread = new_thread_request.clone().into();
    let new_original_post: NewPost = new_thread_request.into();

    let thread: Thread = Thread::create_thread(new_thread, &conn)?;
    let original_post: Post = Post::create_post(new_original_post, &conn)?;
    let user = User::get_user(thread.author_id, &conn)?;

    Ok(Json(thread.into_one_post_thread_response(
        original_post,
        user,
    )))
}

#[put("/lock/<thread_id>")]
fn lock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let thread: Thread = Thread::lock_thread(thread_id, &conn)?;
    Ok(Json(thread.into_full_thread_response(&conn)?))
}

#[put("/unlock/<thread_id>")]
fn unlock_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let thread: Thread = Thread::unlock_thread(thread_id, &conn)?;
    Ok(Json(thread.into_full_thread_response(&conn)?))
}

#[delete("/archive/<thread_id>")]
fn archive_thread(thread_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let thread: Thread = Thread::archive_thread(thread_id, &conn)?;
    Ok(Json(thread.into_full_thread_response(&conn)?))
}

#[get("/get/<forum_id>")]
fn get_threads_by_forum_id(forum_id: i32, conn: Conn) -> Result<Json<Vec<MinimalThreadResponse>>, WeekendAtJoesError> {
    // TODO move the 25 into a parameter
    // TODO make this more efficient by doing a join in the database method
    let threads: Vec<Thread> = Thread::get_threads_in_forum(forum_id, 25, &conn)?;
    threads
        .into_iter()
        .map(|thread| {
            let user: User = User::get_user(thread.author_id, &conn)?;
            let mtr: MinimalThreadResponse = thread.into_minimal_thread_response(
                user,
            );
            Ok(mtr)
        })
        .collect::<Result<Vec<MinimalThreadResponse>, WeekendAtJoesError>>()
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
