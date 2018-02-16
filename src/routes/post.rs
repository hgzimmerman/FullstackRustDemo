use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::forum::{Post, NewPost};
use db::user::User;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::post::{PostResponse, NewPostRequest};
use chrono::Utc;

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


impl Post {
    pub fn into_post_response(self, conn: &Conn) -> Result<PostResponse, WeekendAtJoesError> {
        use db::user::User;
    
        let user: User = User::get_user(self.author_id, conn)?;
        let children: Result<Vec<PostResponse>, WeekendAtJoesError> = self
            .get_post_children(conn)?
            .into_iter()
            .map(|p| Post::into_post_response(p, conn)) // recursion ocurrs here, beware of any performance problems
            .collect();

        let children: Vec<PostResponse> = match children {
            Ok(c) => c,
            Err(e) => return Err(e)
        };

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

impl Routable for Post {
  const ROUTES: &'static Fn() -> Vec<Route> = &||routes![
        ];
    const PATH: &'static str = "/post/";
}