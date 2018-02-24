use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::forum::Forum;
use db::forum::NewForum;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::forum::ForumResponse;
use requests_and_responses::forum::NewForumRequest;
use auth::user_authorization::AdminUser;

impl From<Forum> for ForumResponse {
    fn from(forum: Forum) -> ForumResponse {
        ForumResponse {
            id: forum.id,
            title: forum.title,
            description: forum.description
        }
    }
}

impl From<NewForumRequest> for NewForum {
    fn from(new_forum_request: NewForumRequest) -> NewForum {
        NewForum {
            title: new_forum_request.title,
            description: new_forum_request.description
        }
    }
}



#[get("/forums")]
fn get_forums(conn: Conn) -> Result<Json<Vec<ForumResponse>>, WeekendAtJoesError> {
    Forum::get_forums(&conn)
        .map(|forums| {
            forums.into_iter()
                .map(ForumResponse::from)
                .collect()
        })
        .map(Json)
}

#[post("/create", data = "<new_forum>")]
fn create_forum(new_forum: Json<NewForumRequest>, _admin: AdminUser, conn: Conn) -> Result<Json<ForumResponse>, WeekendAtJoesError> {
    Forum::create_forum(new_forum.into_inner().into(), &conn)
        .map(ForumResponse::from)
        .map(Json)
}



impl Routable for Forum {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_forums, create_forum];
    const PATH: &'static str = "/forum/";
}

