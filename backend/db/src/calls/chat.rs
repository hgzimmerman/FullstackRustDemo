use crate::schema::chats;
use crate::schema::junction_chat_users;
// use diesel::RunQueryDsl;
use crate::user::User;
// use diesel::associations::HasTable;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
// use diesel::Table;
// use diesel::query_dsl::InternalJoinDsl;
use error::JoeResult;
use diesel::PgConnection;
use uuid::Uuid;
use identifiers::chat::ChatUuid;
use identifiers::user::UserUuid;
use crate::calls::prelude::*;
use crate::schema;


#[derive(Debug, Clone, Identifiable, Queryable, ErrorHandler, TypeName)]
#[primary_key(uuid)]
//#[insertable = "NewChat"]
#[table_name = "chats"]
pub struct Chat {
    /// Primary Key.
    pub uuid: Uuid,
    /// The name of the chat
    pub chat_name: String,
    pub leader_uuid: Uuid,
}


#[derive(Insertable, Debug, Clone)]
#[table_name = "chats"]
pub struct NewChat {
    pub chat_name: String,
    pub leader_uuid: Uuid,
}

#[derive(Debug, Clone, Identifiable, Queryable)]
#[table_name = "junction_chat_users"]
pub struct JunctionChatUsers {
    pub id: Uuid,
    pub chat_uuid: Uuid,
    pub user_uuid: Uuid,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "junction_chat_users"]
pub struct ChatUserAssociation {
    pub chat_uuid: Uuid,
    pub user_uuid: Uuid,
}

pub struct ChatData {
    pub chat: Chat,
    pub leader: User,
    pub members: Vec<User>,
}





impl Chat {

    pub fn get_chat(uuid: ChatUuid,conn: &PgConnection) -> JoeResult<Chat> {
        get_row::<Chat,_>(schema::chats::table, uuid.0, conn)
    }
    pub fn delete_chat(uuid: ChatUuid, conn: &PgConnection) -> JoeResult<Chat> {
        delete_row::<Chat,_>(schema::chats::table, uuid.0, conn)
    }
    pub fn create_chat(new: NewChat, conn: &PgConnection) -> JoeResult<Chat> {
        create_row::<Chat, NewChat,_>(schema::chats::table, new, conn)
    }

    pub fn add_user_to_chat(association: ChatUserAssociation, conn: &PgConnection) -> JoeResult<()> {
        use crate::schema::junction_chat_users;

        diesel::insert_into(junction_chat_users::table)
            .values(&association)
            .execute(conn)
            .map_err(Chat::handle_error)?;

        Ok(())
    }

    pub fn remove_user_from_chat(association: ChatUserAssociation, conn: &PgConnection) -> JoeResult<()> {
        use crate::schema::junction_chat_users::dsl::*;
        use crate::schema::junction_chat_users;

        diesel::delete(junction_chat_users::table)
            .filter(chat_uuid.eq(association.chat_uuid))
            .filter(user_uuid.eq(association.user_uuid))
            .execute(conn)
            .map_err(Chat::handle_error)?;
        Ok(())
    }

    fn get_users_in_chat(chat_uuid: ChatUuid, conn: &PgConnection) -> JoeResult<Vec<User>> {
        use crate::schema::junction_chat_users::dsl::junction_chat_users;
        // use schema::users::dsl::*;
        use crate::schema::users;
        use crate::schema::junction_chat_users as junctions;

        junction_chat_users
            .filter(junctions::chat_uuid.eq(chat_uuid.0))
            .inner_join(users::table)
            .select(users::all_columns)
            .load::<User>(conn)
            .map_err(Chat::handle_error)
    }

    pub fn is_user_in_chat(chat_uuid: &ChatUuid, user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<bool> {
        use crate::schema::junction_chat_users::dsl::junction_chat_users;
        use crate::schema::junction_chat_users as junctions;


        let junction = junction_chat_users
            .filter(junctions::user_uuid.eq(user_uuid.0))
            .filter(junctions::chat_uuid.eq(chat_uuid.0))
            .load::<JunctionChatUsers>(conn)
            .map_err(Chat::handle_error)?;
        Ok(junction.get(0).is_some())
    }


    pub fn get_full_chat(chat_uuid: ChatUuid, conn: &PgConnection) -> JoeResult<ChatData> {
        //        let chat_uuid: Uuid = chat_uuid.0;
        let chat: Chat = Chat::get_chat(chat_uuid, &conn)?;
        let leader_uuid = UserUuid(chat.leader_uuid);
        let leader: User = User::get_user(leader_uuid, &conn)?;
        let chat_users: Vec<User> = Chat::get_users_in_chat(chat_uuid, &conn)?;

        Ok(ChatData {
            chat,
            leader,
            members: chat_users,
        })
    }

    pub fn get_chats_user_is_in(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<Vec<Chat>> {
        use crate::schema::junction_chat_users::dsl::junction_chat_users;
        use crate::schema::junction_chat_users as junction;
        // use schema::chats::dsl::*;
        use crate::schema::chats;

        junction_chat_users
            .filter(junction::user_uuid.eq(user_uuid.0))
            .inner_join(chats::table)
            .select(chats::all_columns)
            .load::<Chat>(conn)
            .map_err(Chat::handle_error)
    }
}
