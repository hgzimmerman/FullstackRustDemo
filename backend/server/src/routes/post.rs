use auth_lib::user_authorization::{
    ModeratorUser,
    NormalUser,
};
use db::post::*;
use error::{
    Error,
    VectorMappable,
};
use identifiers::{
    post::PostUuid,
    thread::ThreadUuid,
    user::UserUuid,
};
use pool::Conn;
use rocket::Route;
use rocket_contrib::Json;
use routes::Routable;
use wire::post::{
    EditPostRequest,
    NewPostRequest,
    PostResponse,
};

/// Creates a new post.
/// This operation is available to any user.
#[post("/create", data = "<new_post>")]
fn create_post(
    new_post: Json<NewPostRequest>,
    login_user: NormalUser,
    conn: Conn,
) -> Result<Json<PostResponse>, Error> {
    // check if token user id matches the request user id.
    // This prevents users from creating posts under other user's names.
    let new_post: NewPost = new_post.into_inner().into();
    if new_post.author_uuid != login_user.user_uuid.0 {
        return Err(Error::BadRequest);
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
fn edit_post(
    edit_post_request: Json<EditPostRequest>,
    login_user: NormalUser,
    conn: Conn,
) -> Result<Json<PostResponse>, Error> {
    // Prevent editing other users posts
    let existing_post = Post::get_post(edit_post_request.0.uuid, &conn)?;
    if login_user.user_uuid.0 != existing_post.author_uuid {
        return Err(Error::BadRequest);
    }

    let edit_post_request: EditPostRequest = edit_post_request.into_inner();
    let edit_post_changeset: EditPostChangeset = edit_post_request.clone().into();
    let thread_id: ThreadUuid = edit_post_request.thread_uuid;
    Post::modify_post(edit_post_changeset, thread_id, login_user.user_uuid, &conn)
        .map(PostResponse::from)
        .map(Json)
}

/// Censors a post, preventing it from being seen immediately.
/// This operation is available to moderators.
#[put("/censor/<post_uuid>")]
fn censor_post(post_uuid: PostUuid, _moderator: ModeratorUser, conn: Conn) -> Result<Json<PostResponse>, Error> {
    Post::censor_post(post_uuid, &conn).map(PostResponse::from).map(Json)
}

/// Gets the posts associated with a user.
/// Anyone can perform this operation.
#[get("/users_posts/<user_uuid>")]
fn get_posts_by_user(user_uuid: UserUuid, conn: Conn) -> Result<Json<Vec<PostResponse>>, Error> {
    Post::get_posts_by_user(user_uuid, &conn)
        .map_vec::<PostResponse>()
        .map(Json)
}

impl Routable for Post {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![create_post, censor_post, edit_post, get_posts_by_user];
    const PATH: &'static str = "/post/";
}
