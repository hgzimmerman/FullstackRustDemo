use schema::chats;
use schema::junction_chat_users;
use db::Conn;
use std::ops::Deref;
// use diesel::RunQueryDsl;
use db::user::User;
use diesel::associations::HasTable;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::Table;
// use diesel::query_dsl::InternalJoinDsl;

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

#[derive(Debug, Clone, Identifiable, Queryable)]
// #[insertable = "NewJunctionChatUsers"]
#[table_name = "junction_chat_users"]
pub struct JunctionChatUsers {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "junction_chat_users"]
pub struct NewJunctionChatUsers {
    pub chat_id: i32,
    pub user_id: i32,
}

impl Chat {
    pub fn add_user_to_chat(m_chat_id: i32, m_user_id: i32, conn: &Conn) -> JoeResult<()> {
        // use schema::junction_chat_users::dsl::*;
        use schema::junction_chat_users;

        let junction = NewJunctionChatUsers {
            chat_id: m_chat_id,
            user_id: m_user_id,
        };

        diesel::insert_into(junction_chat_users::table)
            .values(&junction)
            .execute(conn.deref())
            .map_err(Chat::handle_error)?;

        Ok(())
    }

    pub fn get_users_in_chat(m_chat_id: i32, conn: &Conn) -> JoeResult<Vec<User>> {
        use schema::junction_chat_users::dsl::*;
        use schema::users::dsl::*;
        use schema::users;

        junction_chat_users
            .filter(chat_id.eq(m_chat_id))
            .inner_join(users::table)
            .select(users::all_columns)
            .load::<User>(conn.deref())
            .map_err(Chat::handle_error)
    }

    pub fn get_chats_user_is_in(m_user_id: i32, conn: &Conn) -> JoeResult<Vec<Chat>> {
        use schema::junction_chat_users::dsl::*;
        use schema::chats::dsl::*;
        use schema::chats;

        junction_chat_users
            .filter(user_id.eq(m_user_id))
            .inner_join(chats::table)
            .select(chats::all_columns)
            .load::<Chat>(conn.deref())
            .map_err(Chat::handle_error)
    }
}
