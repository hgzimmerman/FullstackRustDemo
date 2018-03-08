use schema::messages;
use db::Conn;
use std::ops::Deref;
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;

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

impl Message {
    fn get_messages_for_chat(m_chat_id: i32, conn: &Conn) -> JoeResult<Vec<Message>> {
        use schema::messages::dsl::*;
        messages
            .filter(chat_id.eq(m_chat_id))
            .order(create_date)
            .load::<Message>(conn.deref())
            .map_err(Message::handle_error)
    }
}
