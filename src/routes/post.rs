use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::post::{Post, NewPost, EditPostChangeset};
use db::Retrievable;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::post::{PostResponse, NewPostRequest, EditPostRequest};
use chrono::Utc;
use auth::user_authorization::NormalUser;
use auth::user_authorization::ModeratorUser;
use db::post::{PostData, ChildlessPostData};

impl From<NewPostRequest> for NewPost {
    fn from(request: NewPostRequest) -> NewPost {
        NewPost {
            thread_id: request.thread_id,
            author_id: request.author_id,
            parent_id: request.parent_id,
            created_date: Utc::now().naive_utc(),
            content: request.content,
            censored: false,
        }
    }
}

impl From<EditPostRequest> for EditPostChangeset {
    fn from(request: EditPostRequest) -> EditPostChangeset {
        EditPostChangeset {
            id: request.id,
            modified_date: Utc::now().naive_utc(),
            content: request.content,
        }
    }
}



impl From<ChildlessPostData> for PostResponse {
    fn from(data: ChildlessPostData) -> PostResponse {
        PostResponse {
            id: data.post.id,
            author: data.user.into(),
            created_date: data.post.created_date,
            modified_date: data.post.modified_date,
            content: data.post.content,
            censored: data.post.censored,
            children: vec![],
        }
    }
}



impl From<PostData> for PostResponse {
    fn from(data: PostData) -> PostResponse {
        PostResponse {
            id: data.post.id,
            author: data.user.into(),
            created_date: data.post.created_date,
            modified_date: data.post.modified_date,
            content: data.post.content,
            censored: data.post.censored,
            children: data.children
                .into_iter()
                .map(PostResponse::from)
                .collect(),
        }
    }
}

impl From<ChildlessPostData> for PostData {
    fn from(childless: ChildlessPostData) -> PostData {
        PostData {
            post: childless.post,
            user: childless.user,
            children: vec![],
        }
    }
}


#[post("/create", data = "<new_post>")]
fn create_post(new_post: Json<NewPostRequest>, login_user: NormalUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    // check if token user id matches the request user id.
    // This prevents users from creating posts under other user's names.
    if new_post.0.author_id != login_user.user_id {
        return Err(WeekendAtJoesError::BadRequest);
    }
    Post::create_and_get_user(new_post.into_inner().into(), &conn)
        .map(PostResponse::from)
        .map(Json)
}


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

#[put("/censor/<post_id>")]
fn censor_post(post_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    Post::censor_post(post_id, &conn)
        .map(PostResponse::from)
        .map(Json)
}

#[get("/users_posts/<user_id>")]
fn get_posts_by_user(user_id: i32, conn: Conn) -> Result<Json<Vec<PostResponse>>, WeekendAtJoesError> {
    Post::get_posts_by_user(user_id, &conn)
        .map(|ok: Vec<ChildlessPostData>| {

            ok.into_iter()
                .map(PostResponse::from)
                .collect::<Vec<PostResponse>>()
        })
        .map(Json)
}


impl Routable for Post {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![create_post, censor_post, edit_post, get_posts_by_user];
    const PATH: &'static str = "/post/";
}
