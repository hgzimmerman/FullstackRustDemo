use schema::forums;
use error::WeekendAtJoesError;
use db::Conn;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use db::handle_diesel_error;

#[derive( Debug, Clone, Identifiable, Queryable)]
#[table_name="forums"]
pub struct Forum {
    pub id: i32,
    pub title: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="forums"]
pub struct NewForum {
    pub title: String,
    pub description: String
}

impl Forum {
    pub fn create_forum(new_forum: NewForum, conn: &Conn) -> Result<Forum, WeekendAtJoesError> {
        use schema::forums;

        diesel::insert_into(forums::table)
            .values(&new_forum)
            .get_result(conn.deref())
            .map_err(|_| WeekendAtJoesError::DatabaseError(None))
    }

    pub fn get_forums(conn: &Conn) -> Result<Vec<Forum>, WeekendAtJoesError> {
        use schema::forums::dsl::*;
        forums
            .load::<Forum>(conn.deref())
            .map_err(|_|  WeekendAtJoesError::DatabaseError(None))
    }

    pub fn get_forum(forum_id: i32, conn: &Conn) -> Result<Forum, WeekendAtJoesError> {
        use schema::forums::dsl::*;

        // Gets the first thread that matches the id.
        forums 
            .find(forum_id)
            .first::<Forum>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Forum"))

    }

}
