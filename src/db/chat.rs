use schema::chats;
use db::Conn;
use std::ops::Deref;
// use diesel::RunQueryDsl;

#[derive(Debug, Clone, Identifiable, Queryable, Crd, ErrorHandler)]
#[insertable = "NewChat"]
#[table_name = "chats"]
pub struct Chat {
    /// Primary Key.
    pub id: i32,
    /// The name of the chat
    pub chat_name: String,
    pub leader_id: i32,
}


#[derive(Insertable, Debug)]
#[table_name = "chats"]
pub struct NewChat {
    pub chat_name: String,
    pub leader_id: i32,
}

impl Chat {}
