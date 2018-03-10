use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;
use db::Retrievable;
use db::Creatable;
use db::forum::Forum;
use db::forum::NewForum;
use error::JoeResult;
use db::Conn;
use requests_and_responses::forum::ForumResponse;
use requests_and_responses::forum::NewForumRequest;
use auth::user_authorization::AdminUser;
use routes::convert_vector;

impl From<Forum> for ForumResponse {
    fn from(forum: Forum) -> ForumResponse {
        ForumResponse {
            id: forum.id,
            title: forum.title,
            description: forum.description,
        }
    }
}

impl From<NewForumRequest> for NewForum {
    fn from(new_forum_request: NewForumRequest) -> NewForum {
        NewForum {
            title: new_forum_request.title,
            description: new_forum_request.description,
        }
    }
}


/// Gets all of the forums.
/// There aren't expected to be many forums, so they aren't paginated.
/// This operation is available to anyone.
#[get("/forums")]
fn get_forums(conn: Conn) -> JoeResult<Json<Vec<ForumResponse>>> {
    Forum::get_all(&conn)
        .map(convert_vector)
        .map(Json)
}

/// Creates a new forum.
/// This operation is available to admins.
#[post("/create", data = "<new_forum>")]
fn create_forum(new_forum: Json<NewForumRequest>, _admin: AdminUser, conn: Conn) -> JoeResult<Json<ForumResponse>> {
    Forum::create(new_forum.into_inner().into(), &conn)
        .map(ForumResponse::from)
        .map(Json)
}



impl Routable for Forum {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_forums, create_forum];
    const PATH: &'static str = "/forum/";
}
