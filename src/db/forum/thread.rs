use schema::threads;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Forum;
use error::WeekendAtJoesError;
use db::Conn;


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

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="threads"]
pub struct NewThread {
    forum_id: i32,
    author_id: i32,
    created_date: NaiveDateTime,
    title: String
}

impl Thread {

    fn create_thread(thread: NewThread, conn: &Conn) -> Result<Thread, WeekendAtJoesError> {
        unimplemented!()
    }

    fn lock_thread(thread_id: i32, conn: &Conn) -> Result<Thread, WeekendAtJoesError>{
        unimplemented!()
    }

    fn archive_thread(thread_id: i32, conn: &Conn) -> Result<Thread, WeekendAtJoesError> {
        unimplemented!()
    }

    fn get_threads_in_forum(forum_id: i32, conn: &Conn) -> Result<Vec<Thread>, WeekendAtJoesError> {
        unimplemented!()
    }

}