use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;
use db::forum::Forum;
use error::JoeResult;
use db::Conn;
use wire::forum::ForumResponse;
use wire::forum::NewForumRequest;
use auth_lib::user_authorization::AdminUser;
use routes::convert_vector;
use identifiers::forum::ForumUuid;


/// Gets all of the forums.
/// There aren't expected to be many forums, so they aren't paginated.
/// This operation is available to anyone.
#[get("/forums")]
fn get_forums(conn: Conn) -> JoeResult<Json<Vec<ForumResponse>>> {
    Forum::get_forums(&conn)
        .map(convert_vector)
        .map(Json)
}

/// Gets a single forum.
#[get("/<forum_uuid>")]
fn get_forum(forum_uuid: ForumUuid, conn: Conn) -> JoeResult<Json<ForumResponse>> {
    Forum::get_forum(forum_uuid, &conn)
        .map(ForumResponse::from)
        .map(Json)
}

/// Creates a new forum.
/// This operation is available to admins.
#[post("/create", data = "<new_forum>")]
fn create_forum(new_forum: Json<NewForumRequest>, _admin: AdminUser, conn: Conn) -> JoeResult<Json<ForumResponse>> {
    Forum::create_forum(new_forum.into_inner().into(), &conn)
        .map(ForumResponse::from)
        .map(Json)
}



impl Routable for Forum {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_forums, create_forum, get_forum];
    const PATH: &'static str = "/forum/";
}
