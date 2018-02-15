use schema::posts;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Thread;
use error::WeekendAtJoesError;
use db::Conn;
use std::ops::Deref;
use diesel;
use diesel::result::Error;
use diesel::RunQueryDsl;
use db::handle_diesel_error;
use diesel::ExpressionMethods;
use diesel::BelongingToDsl;
use diesel::QueryDsl;
use diesel::Identifiable;

#[derive( Debug, Clone, Identifiable, Associations, Queryable)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Thread, foreign_key = "thread_id")]
#[belongs_to(Post, foreign_key = "parent_id")]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub thread_id: i32,
    pub author_id: i32,
    pub parent_id: Option<i32>,
    pub created_date: NaiveDateTime,
    pub modified_date: Option<NaiveDateTime>,
    pub content: String,
    pub censored: bool
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost {
    thread_id: i32,
    author_id: i32,
    parent_id: Option<i32>, // this will always be None, try removing this.
    created_date: NaiveDateTime,
    content: String,
    censored: bool
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPostChangeset {
    id: i32,
    modified_date: NaiveDateTime,
    content: String,
}


impl Post {
    
    fn create_post(new_post: NewPost, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        use schema::posts;

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
    }

    fn modify_post(edit_post_changeset: EditPostChangeset, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        use schema::posts;
        match diesel::update(posts::table)
            .set(&edit_post_changeset)
            .get_result(conn.deref()) 
        {
            Ok(post) => Ok(post),
            Err(e) => Err(handle_diesel_error(e, "Post"))
        }
    }

    fn censor_post(post_id: i32, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        unimplemented!()
    }

    fn get_posts_by_user(user_id: i32, conn: &Conn) -> Result<Vec<Post>, WeekendAtJoesError> {
        unimplemented!()
    }

    fn get_root_post(requested_thread_id: i32, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        use schema::posts::dsl::*;
        use db::forum::Thread;

        let thread: Thread = Thread::get_thread(requested_thread_id, conn)?;

        Post::belonging_to(&thread)
            .filter(
                parent_id.is_null(), // There should only be one thread that has a null parent, and that is the OP/root post
            )
            .limit(1)
            .load::<Post>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
            .and_then(|returned_posts| {
                returned_posts.get(0).cloned().ok_or(WeekendAtJoesError::NotFound {type_name: "Post"})
            })
    }

    pub fn get_post_children(&self, conn: &Conn) -> Result<Vec<Post>, WeekendAtJoesError> {
        use schema::posts::dsl::*;

        Post::belonging_to(self)
            .load::<Post>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
    }

    /// Takes thread id,
    /// gets root post,
    /// gets its children,
    /// for each child, get its children, until no children,
    /// recurse!
    fn get_thread_post_tree() {
        unimplemented!()
    }
}