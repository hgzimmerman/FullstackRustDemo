use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::post::{Post, NewPost, EditPostChangeset};
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::post::{PostResponse, NewPostRequest, EditPostRequest};
use chrono::Utc;
use auth::user_authorization::NormalUser;
use auth::user_authorization::ModeratorUser;

impl From<NewPostRequest> for NewPost {
    fn from(request: NewPostRequest ) -> NewPost {
        NewPost {
            thread_id: request.thread_id,
            author_id: request.author_id,
            parent_id: request.parent_id,
            created_date: Utc::now().naive_utc(),
            content: request.content,
            censored: false 
        }
    }
}

impl From<EditPostRequest> for EditPostChangeset {
    fn from(request: EditPostRequest) -> EditPostChangeset {
        EditPostChangeset {
            id: request.id,
            modified_date: Utc::now().naive_utc(),
            content: request.content
        }
    }
}


impl Post {
    pub fn into_post_response(self, conn: &Conn) -> Result<PostResponse, WeekendAtJoesError> {
        use db::user::User;
    
        let user: User = User::get_user(self.author_id, conn)?;
        let children: Vec<PostResponse> = self
            .get_post_children(conn)?
            .into_iter()
            .map(|p| p.into_post_response(conn)) // recursion ocurrs here, beware of any performance problems
            .collect::<Result<Vec<PostResponse>, WeekendAtJoesError>>()?;

        Ok(PostResponse {
            id: self.id,
            author: user.into(),
            created_date: self.created_date,
            modified_date: self.modified_date,
            content: self.content,
            censored: self.censored,
            children: children
        })
    }

    pub fn into_childless_response(self, user: User) -> PostResponse {
        PostResponse {
            id: self.id,
            author: user.into(),
            created_date: self.created_date,
            modified_date: self.modified_date,
            content: self.content,
            censored: self.censored,
            children: vec!()
        }
    }
}


#[post("/create", data = "<new_post>")]
fn create_post(new_post: Json<NewPostRequest>, login_user: NormalUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    // check if token user id matches the request user id.
    // This prevents users from creating posts under other user's names.
    if new_post.0.author_id != login_user.user_id {
        return Err(WeekendAtJoesError::BadRequest)
    }
    let user: User = User::get_user(new_post.author_id, &conn)?;
    Post::create_post(new_post.into_inner().into(), &conn)
        .map(|post| post.into_childless_response(user))
        .map(Json)
}


#[put("/edit", data = "<edit_post_request>")]
fn edit_post(edit_post_request: Json<EditPostRequest>, login_user: NormalUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    // Prevent editing other users posts
    let existing_post = Post::get_post_by_id(edit_post_request.0.id , &conn)?;
    if login_user.user_id != existing_post.author_id {
        return Err(WeekendAtJoesError::BadRequest)
    }

    let edit_post_request: EditPostRequest = edit_post_request.into_inner();
    let edit_post_changeset: EditPostChangeset = edit_post_request.clone().into();
    let thread_id: i32 = edit_post_request.thread_id;
    let user: User = Post::get_user_by_post(edit_post_changeset.id, &conn)?;
    Post::modify_post(edit_post_changeset, thread_id, &conn)
        .map(|post| post.into_childless_response(user))
        .map(Json)
}

#[put("/censor/<post_id>")]
fn censor_post(post_id: i32, _moderator: ModeratorUser, conn: Conn) -> Result<Json<PostResponse>, WeekendAtJoesError> {
    let user: User = Post::get_user_by_post(post_id, &conn)?;
    Post::censor_post(post_id, &conn)
        .map(|post| post.into_childless_response(user))
        .map(Json)
}

#[get("/users_posts/<user_id>")]
fn get_posts_by_user(user_id: i32, conn: Conn) -> Result<Json<Vec<PostResponse>>, WeekendAtJoesError> {
    let user: User = User::get_user(user_id, &conn)?;
    Post::get_posts_by_user(user_id, &conn)
        .map(|posts| {
            posts
                .into_iter()
                .map(|post| post.into_childless_response(user.clone()))
                .collect()
        })
        .map(Json)
}


impl Routable for Post {
  const ROUTES: &'static Fn() -> Vec<Route> = &||routes![
        create_post,
        censor_post,
        edit_post,
        get_posts_by_user
    ];
    const PATH: &'static str = "/post/";
}