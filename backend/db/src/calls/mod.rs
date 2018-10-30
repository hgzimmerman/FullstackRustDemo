//! Calls to the database.

pub mod auth;
pub mod user;
pub mod article;
pub mod forum;
pub mod thread;
pub mod post;
pub mod bucket;
pub mod question;
pub mod answer;
pub mod chat;
pub mod message;




use diesel::dsl::Find;
use diesel::pg::PgConnection;
use diesel::query_dsl::{LoadQuery, RunQueryDsl};
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::result::Error;
use uuid::Uuid;
use diesel::insertable::Insertable;
use diesel::query_source::Queryable;
use diesel::pg::Pg;
use diesel::associations::Identifiable;
use diesel::query_builder::IntoUpdateTarget;
use diesel::associations::HasTable;
use diesel::query_source::Table;
use diesel::query_builder::QueryId;
use diesel::query_builder::QueryFragment;
use crate::schema;
use crate::User;
use identifiers::user::UserUuid;
use diesel::Expression;

pub fn get_row<'a, Model, Table>(table: Table, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
    where
        Table: FindDsl<Uuid>,
        Find<Table, Uuid>: LoadQuery<PgConnection, Model>,
{
    table
        .find(uuid)
        .get_result::<Model>(conn)
}

fn get_user(uuid: UserUuid,conn: &PgConnection) -> Result<User, Error> {
    get_row::<User,_>(schema::users::table, uuid.0, conn)
}



pub fn delete_row<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
    where
//
        Model: Identifiable<Table = Tab>
            + QueryId
            + QueryFragment<Pg>
            + Queryable< <<Tab as Table>::AllColumns as Expression>::SqlType, Pg>,
        Tab: Table + FindDsl<Uuid> + QueryId,
        Find<Tab, Uuid>: IntoUpdateTarget<Table = Tab, WhereClause = Model>, // This is known to be good with no action
        <Tab as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<Pg>, // This is known to work with execute
        <Tab as diesel::Table>::AllColumns: QueryId,
        <Tab as diesel::Table>::AllColumns: QueryFragment<Pg>,
        Find<Tab, Uuid>: LoadQuery<PgConnection, Model>,
        diesel::pg::Pg: diesel::sql_types::HasSqlType<<<Tab as diesel::Table>::AllColumns as diesel::Expression>::SqlType>,
{
//    let target = table.find(id);
    diesel::delete(table.find(uuid))
        .get_result::<Model>(conn)
}


//pub fn delete_row_2<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
//    where
////
//        Model: Queryable<Tab::SqlType, Pg>,
//        Tab: Table
//            + FindDsl<Uuid>,
//        Find<Tab, Uuid>: LoadQuery<PgConnection, Model>,
//        <Tab as diesel::query_dsl::filter_dsl::FindDsl<uuid::Uuid>>::Output: IntoUpdateTarget,
//        Model: diesel::Queryable<<<<<Tab as diesel::query_dsl::filter_dsl::FindDsl<uuid::Uuid>>::Output as diesel::associations::HasTable>::Table as diesel::Table>::AllColumns as diesel::Expression>::SqlType, diesel::pg::Pg>
////        <Tab as FindDsl<uuid::Uuid>>::Output: diesel::Identifiable,
////        <Tab as FindDsl<Uuid>>::Output: HasTable,
//{
////    let target = table.find(id);
//    diesel::delete(table.find(uuid))
//        .get_result(conn)
//}

//fn delete_user(uuid: UserUuid, conn: &PgConnection) -> Result<User, Error> {
//    delete_row::<User,_>(schema::users::table, uuid.0, conn)
//}

//pub fn create_row<'a, Insert, Model, Table>(table: Table, value: &Insert, conn: &PgConnection) -> Result<Model, Error>
//    where
////        Insert: Insertable<Table>, //+ Queryable<Table, Pg>,
////        Model: LoadQuery<PgConnection, Model>,
////        Table: diesel::Table
//{
//        diesel::insert_into(table)
//            .values(value)
//            .get_result::<Model>(conn)
////            .execute(conn);
//
////            .get_result::<Model>(conn)
//}

