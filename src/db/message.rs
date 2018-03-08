use schema::messages;
use db::Conn;
use std::ops::Deref;
use chrono::NaiveDateTime;
// use diesel::RunQueryDsl;

#[derive(Debug, Clone, Identifiable, Queryable, Crd, ErrorHandler)]
#[insertable = "NewMessage"]
#[table_name = "messages"]
pub struct Message {
    /// Primary Key.
    pub id: i32,
    pub author_id: i32,
    pub chat_id: i32,
    pub reply_id: Option<i32>,
    pub message_content: String,
    pub read_flag: bool,
    pub create_date: NaiveDateTime,
}


#[derive(Insertable, Debug)]
#[table_name = "messages"]
pub struct NewMessage {
    pub author_id: i32,
    pub chat_id: i32,
    pub reply_id: Option<i32>,
    pub message_content: String,
    pub read_flag: bool,
    pub create_date: NaiveDateTime,
}

impl Message {}
