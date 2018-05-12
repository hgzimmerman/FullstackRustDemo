use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::post::*;
use db::Retrievable;
use error::WeekendAtJoesError;
use db::Conn;
use wire::post::{PostResponse, NewPostRequest, EditPostRequest};
use auth::user_authorization::NormalUser;
use auth::user_authorization::ModeratorUser;
use error::VectorMappable;



/// Creates a new post.
/// This operation is available to any user.
#[post("/create", data = "<new_post>")]
fn create_post(new_post: Json<NewPostRequest>, login_user: NormalUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    // check if token user id matches the request user id.
    // This prevents users from creating posts under other user's names.
    let new_post: NewPost = new_post.into_inner().into();
    if new_post.author_id != login_user.user_id {
        return Err(WeekendAtJoesError::BadRequest);
    }
    Post::create_and_get_user(new_post, &conn)
        .map(PostResponse::from)
        .map(Json)
}

/// This edits posts.
/// This operation is available to users.
/// This will only work if the user is the author of the post.
/// The returned PostResponse will not have children and therefore the client must merge the new data.
#[put("/edit", data = "<edit_post_request>")]
fn edit_post(edit_post_request: Json<EditPostRequest>, login_user: NormalUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    // Prevent editing other users posts
    let existing_post = Post::get_by_id(edit_post_request.0.id, &conn)?;
    if login_user.user_id != existing_post.author_id {
        return Err(WeekendAtJoesError::BadRequest);
    }

    let edit_post_request: EditPostRequest = edit_post_request.into_inner();
    let edit_post_changeset: EditPostChangeset = edit_post_request.clone().into();
    let thread_id: i32 = edit_post_request.thread_id;
    Post::modify_post(edit_post_changeset, thread_id, &conn)
        .map(PostResponse::from)
        .map(Json)
}

/// Censors a post, preventing it from being seen immediately.
/// This operation is available to moderators.
#[put("/censor/<post_id>")]
fn censor_post(post_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    Post::censor_post(post_id, &conn)
        .map(PostResponse::from)
        .map(Json)
}

/// Gets the posts associated with a user.
/// Anyone can perform this operation.
#[get("/users_posts/<user_id>")]
fn get_posts_by_user(user_id: i32, conn: Conn) -> Result<Json<Vec<PostResponse>>, WeekendAtJoesError> {
    Post::get_posts_by_user(user_id, &conn)
        .map_vec::<PostResponse>()
        .map(Json)
}


impl Routable for Post {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![create_post, censor_post, edit_post, get_posts_by_user];
    const PATH: &'static str = "/post/";
}
