use schema::posts;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Thread;


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
    content: String
}