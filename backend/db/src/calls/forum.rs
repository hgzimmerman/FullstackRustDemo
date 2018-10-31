use crate::schema::forums;
use uuid::Uuid;
use diesel::pg::PgConnection;
use identifiers::forum::ForumUuid;
use error::JoeResult;
use crate::calls::prelude::*;
use crate::schema;

#[derive(Debug, Clone, Identifiable, Queryable, CrdUuid, ErrorHandler, TypeName)]
#[primary_key(uuid)]
#[insertable = "NewForum"]
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
    pub fn get_forum(uuid: ForumUuid,conn: &PgConnection) -> JoeResult<Forum> {
        get_row::<Forum,_>(schema::forums::table, uuid.0, conn)
    }
    pub fn delete_forum(uuid: ForumUuid, conn: &PgConnection) -> JoeResult<Forum> {
        delete_row::<Forum,_>(schema::forums::table, uuid.0, conn)
    }
    pub fn create_forum(new: NewForum, conn: &PgConnection) -> JoeResult<Forum> {
        create_row::<Forum, NewForum,_>(schema::forums::table, new, conn)
    }
}