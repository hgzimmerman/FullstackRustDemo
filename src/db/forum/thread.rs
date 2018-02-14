use schema::threads;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Forum;


#[derive( Debug, Clone, Identifiable, Associations, Queryable)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Forum, foreign_key = "forum_id")]
#[table_name="threads"]
pub struct Thread {
    id: i32,
    forum_id: i32,
    author_id: i32,
    created_date: NaiveDateTime,
    title: String
}