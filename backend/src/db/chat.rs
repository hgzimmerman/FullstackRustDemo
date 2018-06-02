use schema::chats;
use schema::junction_chat_users;
// use diesel::RunQueryDsl;
use db::user::User;
// use diesel::associations::HasTable;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
// use diesel::Table;
// use diesel::query_dsl::InternalJoinDsl;
use error::JoeResult;
use diesel::PgConnection;

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


#[derive(Insertable, Debug, Clone)]
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

#[derive(Insertable, Debug, Clone)]
#[table_name = "junction_chat_users"]
pub struct ChatUserAssociation {
    pub chat_id: i32,
    pub user_id: i32,
}

pub struct ChatData {
    pub chat: Chat,
    pub leader: User,
    pub members: Vec<User>,
}

impl Chat {
    pub fn add_user_to_chat(association: ChatUserAssociation, conn: &PgConnection) -> JoeResult<()> {
        use schema::junction_chat_users;

        diesel::insert_into(junction_chat_users::table)
            .values(&association)
            .execute(conn)
            .map_err(Chat::handle_error)?;

        Ok(())
    }

    pub fn remove_user_from_chat(association: ChatUserAssociation, conn: &PgConnection) -> JoeResult<()> {
        use schema::junction_chat_users::dsl::*;
        use schema::junction_chat_users;

        diesel::delete(junction_chat_users::table)
            .filter(chat_id.eq(association.chat_id))
            .filter(user_id.eq(association.user_id))
            .execute(conn)
            .map_err(Chat::handle_error)?;
        Ok(())
    }

    fn get_users_in_chat(m_chat_id: i32, conn: &PgConnection) -> JoeResult<Vec<User>> {
        use schema::junction_chat_users::dsl::*;
        // use schema::users::dsl::*;
        use schema::users;

        junction_chat_users
            .filter(chat_id.eq(m_chat_id))
            .inner_join(users::table)
            .select(users::all_columns)
            .load::<User>(conn)
            .map_err(Chat::handle_error)
    }

    pub fn is_user_in_chat(m_chat_id: i32, m_user_id: i32, conn: &PgConnection) -> JoeResult<bool> {
        use schema::junction_chat_users::dsl::*;
        let junction = junction_chat_users
            .filter(user_id.eq(m_user_id))
            .filter(chat_id.eq(m_chat_id))
            .load::<JunctionChatUsers>(conn)
            .map_err(Chat::handle_error)?;
        Ok(junction.get(0).is_some())
    }

    pub fn get_full_chat(chat_id: i32, conn: &PgConnection) -> JoeResult<ChatData> {
        let chat: Chat = Chat::get_by_id(chat_id, &conn)?;
        let leader: User = User::get_by_id(chat.leader_id, &conn)?;
        let chat_users: Vec<User> = Chat::get_users_in_chat(chat_id, &conn)?;

        Ok(ChatData {
            chat,
            leader,
            members: chat_users,
        })
    }

    pub fn get_chats_user_is_in(m_user_id: i32, conn: &PgConnection) -> JoeResult<Vec<Chat>> {
        use schema::junction_chat_users::dsl::*;
        // use schema::chats::dsl::*;
        use schema::chats;

        junction_chat_users
            .filter(user_id.eq(m_user_id))
            .inner_join(chats::table)
            .select(chats::all_columns)
            .load::<Chat>(conn)
            .map_err(Chat::handle_error)
    }
}
