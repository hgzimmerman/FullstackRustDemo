use chrono::NaiveDateTime;
use crate::{
    calls::prelude::*,
    chat::Chat,
    diesel_extensions::pagination::*,
    schema::{
        self,
        messages,
    },
    user::User,
};
use diesel::PgConnection;
use error::BackendResult;
use identifiers::{
    chat::ChatUuid,
    message::MessageUuid,
    user::UserUuid,
};
use uuid::Uuid;

#[derive(Debug, Clone, Identifiable, Queryable, Associations, TypeName)]
#[primary_key(uuid)]
#[belongs_to(Message, foreign_key = "reply_uuid")]
#[belongs_to(User, foreign_key = "author_uuid")]
#[belongs_to(Chat, foreign_key = "chat_uuid")]
#[table_name = "messages"]
pub struct Message {
    /// Primary Key.
    pub uuid: Uuid,
    pub author_uuid: Uuid,
    pub chat_uuid: Uuid,
    pub reply_uuid: Option<Uuid>,
    pub message_content: String,
    pub read_flag: bool,
    pub create_date: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "messages"]
pub struct NewMessage {
    pub author_uuid: Uuid,
    pub chat_uuid: Uuid,
    pub reply_uuid: Option<Uuid>,
    pub message_content: String,
    pub read_flag: bool,
    pub create_date: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct MessageData {
    pub message: Message,
    pub author: User,
    pub reply: Option<Box<MessageData>>,
}

impl Message {
    pub fn get_message_simple(uuid: MessageUuid, conn: &PgConnection) -> BackendResult<Message> {
        get_row::<Message, _>(schema::messages::table, uuid.0, conn)
    }
    pub fn delete_message(uuid: MessageUuid, conn: &PgConnection) -> BackendResult<Message> {
        delete_row::<Message, _>(schema::messages::table, uuid.0, conn)
    }
    pub fn create_message_simple(new: NewMessage, conn: &PgConnection) -> BackendResult<Message> {
        create_row::<Message, NewMessage, _>(schema::messages::table, new, conn)
    }
    // pub fn get_paginated(m_chat_id: i32, page_index: i32, page_size: i32, conn: &Conn) -> JoeResult<Vec<Message>> {
    //     use schema::messages::dsl::*;
    //     use schema::messages;
    //     use db::diesel_extensions::pagination::Paginate;
    //     use diesel::query_builder::Query;
    //     use diesel::prelude::*;
    //     use schema::users::dsl::*;
    //     use schema::users;

    //     messages::table
    //         .inner_join(users::table)
    //         .filter(chat_id.eq(m_chat_id))
    //         // .filter(users::id.gt(0))
    //         .select((messages::all_columns, users::all_columns))
    //         .paginate(page_index.into())
    //         .per_page(page_size.into())
    //         .load_and_count_pages::<(Message, User)>(conn.deref())
    //         .map_err(handle_err::<Message>);

    //     // users::table
    //     //     .inner_join(messages::table)
    //     //     .filter(users::id.eq(m_chat_id))
    //     //     // .filter(users::id.gt(0))
    //     //     .select((users::all_columns, messages::all_columns))
    //     //     .paginate(page_index.into())
    //     //     .per_page(page_size.into())
    //     //     .load_and_count_pages::<(User, Message)>(conn.deref())
    //     //     .map_err(handle_err::<Message>);

    //     unimplemented!()
    // }

    pub fn create_message(new_message: NewMessage, conn: &PgConnection) -> BackendResult<MessageData> {
        let message = Message::create_message_simple(new_message, conn)?;
        let author_uuid = UserUuid(message.author_uuid);
        let author = User::get_user(author_uuid, conn)?;
        // Get only the first reply in the possible chain of replies.
        if message.reply_uuid.is_some() {
            // The unwrap is safe because the if_some condition was checked above
            let reply_message_uuid: MessageUuid = MessageUuid(message.reply_uuid.unwrap());
            let reply = Message::get_message(reply_message_uuid, false, conn)?;
            Ok(MessageData {
                message,
                author,
                reply: Some(Box::new(reply)),
            })
        } else {
            Ok(MessageData {
                message,
                author,
                reply: None,
            })
        }
    }

    fn get_message(uuid: MessageUuid, with_reply: bool, conn: &PgConnection) -> BackendResult<MessageData> {
        let message = Message::get_message_simple(uuid, conn)?;
        let author_uuid = UserUuid(message.author_uuid);
        let author = User::get_user(author_uuid, conn)?;
        // If the parameter instructs to get the reply, and the reply id exists, get it.
        if with_reply && message.reply_uuid.is_some() {
            // The unwrap is safe because the if_some condition was checked above
            let reply_message_uuid: MessageUuid = MessageUuid(message.reply_uuid.unwrap());
            let reply: MessageData = Message::get_message(reply_message_uuid, with_reply, conn)?;
            Ok(MessageData {
                message,
                author,
                reply: Some(Box::new(reply)),
            })
        } else {
            Ok(MessageData {
                message,
                author,
                reply: None,
            })
        }
    }

    pub fn get_messages_for_chat(
        chat_uuid: ChatUuid,
        page_index: i32,
        page_size: i32,
        conn: &PgConnection,
    ) -> BackendResult<Vec<MessageData>> {
        //        use schema::messages::dsl::*;
        use crate::schema::{
            messages,
            users,
        };
        use diesel::prelude::*;

        //        let m_chat_id: Uuid = m_chat_id.0;

        let (messages_and_users, _count): (Vec<(Message, User)>, i64) = messages::table
            .inner_join(users::table)
            .order(messages::create_date)
            .filter(messages::chat_uuid.eq(chat_uuid.0))
            .select((messages::all_columns, users::all_columns))
            .paginate(page_index.into())
            .per_page(page_size.into())
            .load_and_count_pages::<(Message, User)>(conn) // Apparently just `load` doesn't work, so we use this instead and throw away the count.
            .map_err(handle_err::<Message>)?;

        let collected_messages: Vec<Message> = messages_and_users.iter().map(|x| x.0.clone()).collect();

        // not every message will have a corresponding reply, so this vec should be sparce
        let replied = Message::belonging_to(&collected_messages) // I'm not 100% sure that this gets the intended messages. Write a test to check.
            .inner_join(users::table)
            .load::<(Message, User)>(conn)
            .map_err(handle_err::<Message>)?
            .grouped_by(&collected_messages);

        let message_data = messages_and_users
            .into_iter()
            .zip(replied)
            .map(|x: ((Message, User), Vec<(Message, User)>)| MessageData {
                message: (x.0).0,
                author: (x.0).1,
                reply: (x.1)
                    .get(0)
                    .cloned()
                    .map(|y| MessageData {
                        message: y.0,
                        author: y.1,
                        reply: None,
                    })
                    .map(Box::new),
            })
            .collect::<Vec<MessageData>>();

        Ok(message_data)
    }
}
