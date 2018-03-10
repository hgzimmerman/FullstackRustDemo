use schema::messages;
use db::Conn;
use std::ops::Deref;
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use db::user::User;
use diesel::BelongingToDsl;
use db::chat::Chat;

#[derive(Debug, Clone, Identifiable, Queryable, Associations, Crd, ErrorHandler)]
#[belongs_to(Message, foreign_key = "reply_id")]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Chat, foreign_key = "chat_id")]
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

pub struct MessageData {
    pub message: Message,
    pub author: User,
    pub reply: Option<Box<MessageData>>,
}

use diesel::JoinOnDsl;
use diesel::query_dsl::InternalJoinDsl;
use diesel::NullableExpressionMethods;
use diesel::GroupedBy;
impl Message {
    pub fn get_messages_for_chat(m_chat_id: i32, conn: &Conn) -> JoeResult<Vec<MessageData>> {
        use schema::messages::dsl::*;
        use schema::messages;
        use schema::users;

        let messages_and_users: Vec<(Message, User)> = messages
            .inner_join(users::table)
            .filter(chat_id.eq(m_chat_id))
            .order(create_date)
            .load::<(Message, User)>(conn.deref())
            .map_err(Message::handle_error)?;

        let collected_messages: Vec<Message> = messages_and_users
            .iter()
            .map(|x| x.0.clone())
            .collect();

        // not every message will have a corresponding reply, so this vec should be sparce
        let replied = Message::belonging_to(&collected_messages) // I'm not 100% sure that this gets the intended messages. Write a test to check.
            .inner_join(users::table)
            .load::<(Message, User)>(conn.deref())
            .map_err(Message::handle_error)?
            .grouped_by(&collected_messages);

        let message_data = messages_and_users
            .into_iter()
            .zip(replied)
            .map(|x: ((Message,User), Vec<(Message, User)>) |{
                MessageData{
                    message: (x.0).0,
                    author: (x.0).1,
                    reply: (x.1).get(0).cloned()
                    .map(|y| MessageData{
                        message: y.0,
                        author: y.1,
                        reply: None
                    })
                    .map(Box::new)
                }
            })
            .collect::<Vec<MessageData>>();

        Ok(message_data)


    }
}
