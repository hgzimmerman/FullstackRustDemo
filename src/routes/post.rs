use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::forum::Post;
use error::WeekendAtJoesError;
use db::Conn;
use requests_and_responses::post::PostResponse;

impl From<Post> for PostResponse {
    fn from(post: Post) -> PostResponse {
        // PostResponse {
        //     id: post.id,
        // }
        unimplemented!()
    }
}

impl Post {
    fn to_post_response(post: Post, conn: &Conn) -> Result<PostResponse, WeekendAtJoesError> {
        use db::user::User;
    
        let user: User = User::get_user(post.author_id, conn)?;
        let children: Result<Vec<PostResponse>, WeekendAtJoesError>= post
        .get_post_children(conn)?
        .into_iter()
        .map(|p| Post::to_post_response(p, conn))
        .collect();

        let children: Vec<PostResponse> = match children {
            Ok(c) => c,
            Err(e) => return Err(e)
        };

        Ok(PostResponse {
            id: post.id,
            author: user.into(),
            created_date: post.created_date,
            modified_date: post.modified_date,
            content: post.content,
            censored: post.censored,
            children:children
        })
    }
}