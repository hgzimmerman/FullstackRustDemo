use schema::posts;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Thread;
use error::WeekendAtJoesError;
use db::Conn;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use db::handle_diesel_error;
use diesel::ExpressionMethods;
use diesel::BelongingToDsl;
use diesel::QueryDsl;

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
    pub thread_id: i32,
    pub author_id: i32,
    pub parent_id: Option<i32>, // this will always be None, try removing this.
    pub created_date: NaiveDateTime,
    pub content: String,
    pub censored: bool
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="posts"]
pub struct EditPostChangeset {
    pub id: i32,
    pub modified_date: NaiveDateTime,
    pub content: String,
}


impl Post {
    
    pub fn create_post(new_post: NewPost, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        use schema::posts;

        let target_thread = Thread::get_thread(new_post.thread_id, conn)?;
        if target_thread.locked {
            return Err(WeekendAtJoesError::ThreadLocked)
        }

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
    }

    pub fn modify_post(edit_post_changeset: EditPostChangeset, thread_id: i32, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        use schema::posts;

        let target_thread = Thread::get_thread(thread_id, conn)?;
        if target_thread.locked {
            return Err(WeekendAtJoesError::ThreadLocked)
        }

        diesel::update(posts::table)
            .set(&edit_post_changeset)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
    }

    pub fn censor_post(post_id: i32, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        use schema::posts::dsl::*;
        use schema::posts;
        diesel::update(posts::table)
            .filter(id.eq(post_id))
            .set(censored.eq(true))
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
    }

    pub fn get_posts_by_user(user_id: i32, conn: &Conn) -> Result<Vec<Post>, WeekendAtJoesError> {
        let user: User = User::get_user(user_id, conn)?;

        Post::belonging_to(&user)
            .load::<Post>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
    }

    pub fn get_user_by_post(post_id: i32, conn: &Conn) -> Result<User, WeekendAtJoesError> {
        use schema::posts::dsl::*;
        use schema::users::dsl::*;
        let post: Post = posts 
            .find(post_id)
            .first::<Post>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))?;

        users
            .find(post.author_id)
            .first(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))
    }

    pub fn get_root_post(requested_thread_id: i32, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
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
        Post::belonging_to(self)
            .load::<Post>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Post"))
    }

}