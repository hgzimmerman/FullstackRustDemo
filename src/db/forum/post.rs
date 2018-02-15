use schema::posts;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Thread;
use error::WeekendAtJoesError;
use db::Conn;

#[derive( Debug, Clone, Identifiable, Associations, Queryable)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Thread, foreign_key = "thread_id")]
#[belongs_to(Post, foreign_key = "parent_id")]
#[table_name="posts"]
pub struct Post {
    id: i32,
    thread_id: i32,
    author_id: i32,
    parent_id: Option<i32>,
    created_date: NaiveDateTime,
    modified_date: Option<NaiveDateTime>,
    content: String,
    censored: bool
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost {
    thread_id: i32,
    author_id: i32,
    // parent_id: Option<i32>, // this will always be None, try removing this.
    content: String,
    censored: bool
}

pub struct EditPostChangeset {
    id: i32,
    content: String
}


impl Post {
    
    fn create_post(new_post: NewPost, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        unimplemented!()
    }

    fn modify_post(edit_post_changeset: EditPostChangeset, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        unimplemented!()
    }

    fn censor_post(post_id: i32, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        unimplemented!()
    }

    fn get_posts_by_user(user_id: i32, conn: &Conn) -> Result<Vec<Post>, WeekendAtJoesError> {
        unimplemented!()
    }

    // This is just here for now, 
    // In the future, there will be some 'recursive' get post children function that can be called to construct a tree.
    fn get_post_tree_by_thread_id(thread_id: i32, conn: &Conn) -> Result<Post, WeekendAtJoesError> {
        unimplemented!()
    }
}