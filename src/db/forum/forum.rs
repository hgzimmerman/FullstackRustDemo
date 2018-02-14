use schema::forums;
use error::WeekendAtJoesError;
use db::Conn;
use diesel::result::Error;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;

#[derive( Debug, Clone, Identifiable, Queryable)]
#[table_name="forums"]
pub struct Forum {
    id: i32,
    title: String
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="forums"]
pub struct NewForum {
    title: String
}

impl Forum {
    pub fn create_forum(new_forum: NewForum, conn: &Conn) -> Result<Forum, WeekendAtJoesError> {
        use schema::forums;

        diesel::insert_into(forums::table)
            .values(&new_forum)
            .get_result(conn.deref())
            .map_err(|e| {
                match e {
                    Error::NotFound => WeekendAtJoesError::NotFound { type_name: "Forum"},
                    _ => WeekendAtJoesError::DatabaseError(None),
                }
            })
    }

}
