use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::forum::{Thread, NewThread};
use db::forum::{Post, NewPost};
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::thread::{NewThreadRequest, ThreadResponse};
use chrono::Utc;


impl From<NewThreadRequest> for NewThread {
    fn from(request: NewThreadRequest) -> NewThread {
        NewThread {
            forum_id: request.forum_id,
            author_id: request.author_id,
            created_date: Utc::now().naive_utc(),
            locked: false,
            archived: false,
            title: request.title 
        }
    }
} 

impl From<NewThreadRequest> for NewPost {
    fn from(request: NewThreadRequest) -> NewPost {
        // NewPost::from(request.post)
        unimplemented!()
    }
} 


impl Thread {
    /// The response requires both a post and a user to be attached.
    fn into_thread_response(self, post: Post, user: User) -> ThreadResponse {
        ThreadResponse {
            id: self.id,
            title: self.title,
            author: user.clone().into(),
            posts: post.into_childless_response(user),
            created_date: self.created_date,
            locked: self.locked,
        }
    }
}


#[post("/create", data = "<new_thread_request>")]
fn create_thread(new_thread_request: Json<NewThreadRequest>, conn: Conn) -> Result<Json<ThreadResponse>, WeekendAtJoesError> {
    let new_thread_request = new_thread_request.into_inner();

    let new_thread: NewThread = new_thread_request.clone().into();
    let new_original_post: NewPost = new_thread_request.into();

    let thread: Thread = Thread::create_thread(new_thread, &conn)?;
    let original_post: Post = Post::create_post(new_original_post, &conn)?;
    let user = User::get_user(thread.author_id, &conn)?;

    Ok(Json(thread.into_thread_response( original_post, user)))
}


impl Routable for Thread {
  const ROUTES: &'static Fn() -> Vec<Route> = &||routes![
          create_thread 
        ];
    const PATH: &'static str = "/thread/";
}