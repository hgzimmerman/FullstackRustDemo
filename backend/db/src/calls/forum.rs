use uuid::Uuid;
use diesel::pg::PgConnection;
use identifiers::forum::ForumUuid;
use error::BackendResult;
use crate::{
    calls::prelude::*,
    schema::forums,
    schema
};

#[derive(Debug, Clone, Identifiable, Queryable, TypeName)]
#[primary_key(uuid)]
#[table_name = "forums"]
pub struct Forum {
    /// Primary Key.
    pub uuid: Uuid,
    /// Displayed title of the forum
    pub title: String,
    /// The description that informs users what topics should be discussed in the forum.
    pub description: String,
}

#[derive(Insertable, Debug)]
#[table_name = "forums"]
pub struct NewForum {
    pub title: String,
    pub description: String,
}

impl Forum {
    pub fn get_forum(uuid: ForumUuid,conn: &PgConnection) -> BackendResult<Forum> {
        get_row::<Forum,_>(schema::forums::table, uuid.0, conn)
    }
    pub fn get_forums(conn: &PgConnection) -> BackendResult<Vec<Forum>> {
        get_rows::<Forum,_>(schema::forums::table, conn)
    }
    pub fn delete_forum(uuid: ForumUuid, conn: &PgConnection) -> BackendResult<Forum> {
        delete_row::<Forum,_>(schema::forums::table, uuid.0, conn)
    }
    pub fn create_forum(new: NewForum, conn: &PgConnection) -> BackendResult<Forum> {
        create_row::<Forum, NewForum,_>(schema::forums::table, new, conn)
    }
}