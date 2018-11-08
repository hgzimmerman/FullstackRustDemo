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


use uuid::Uuid;
use diesel::{
    query_builder::IntoUpdateTarget,
    pg::Pg,
    query_source::Queryable,
    insertable::Insertable,
    result::Error as DieselError,
    query_dsl::filter_dsl::FindDsl,
    query_dsl::{LoadQuery, RunQueryDsl},
    pg::PgConnection,
    dsl::Find,
    associations::HasTable,
    query_source::Table,
    query_builder::QueryId,
    query_builder::QueryFragment,
    Expression,
    query_builder::AsQuery,
    delete,
    query_builder::DeleteStatement,
    sql_types::HasSqlType,
    query_builder::InsertStatement,
    query_builder::AsChangeset,
    query_source::QuerySource,
    helper_types::Update
};
use typename::TypeName;
use error::Error;



pub mod prelude {
    pub use super::get_row;
    pub use super::get_rows;
    pub use super::delete_row;
    pub use super::create_row;
    pub use super::update_row;
    pub use super::handle_err;
}

pub fn handle_err<T: TypeName>(error: DieselError) -> Error {
    match error {
        DieselError::NotFound => Error::NotFound { type_name: T::type_name() },
        _ => Error::DatabaseError(Some(format!("{:?}", error))), // This gives some insight into what the internal state of the app is. Set this to none when this enters production.
    }
}

/// Generic function for getting a whole row from a given table.
#[inline(always)]
pub fn get_row<'a, Model, Table>(table: Table, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
    where
        Table: FindDsl<Uuid>,
        Find<Table, Uuid>: LoadQuery<PgConnection, Model>,
        Model: TypeName
{
    table
        .find(uuid)
        .get_result::<Model>(conn)
        .map_err(handle_err::<Model>)
}



#[inline(always)]
pub fn get_rows<'a, Model, Table>(table: Table, conn: &PgConnection) -> Result<Vec<Model>, Error>
    where
    Table: RunQueryDsl<Model> + LoadQuery<PgConnection, Model>,
    Model: TypeName
{
    table.load::<Model>(conn)
        .map_err(handle_err::<Model>)
}



/// Generic function for deleting a row from a given table.
#[inline(always)]
pub fn delete_row<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
    where
        Tab: FindDsl<Uuid> + Table,
        <Tab as FindDsl<Uuid>>::Output: IntoUpdateTarget,
        Pg: HasSqlType<<<<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns as Expression>::SqlType>,
        <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryId,
        <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryFragment<Pg>,
        DeleteStatement<<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table, <<Tab as FindDsl<Uuid>>::Output as IntoUpdateTarget>::WhereClause>: LoadQuery<PgConnection, Model>,
        Model: TypeName
{
    delete(table.find(uuid))
        .get_result::<Model>(conn)
        .map_err(handle_err::<Model>)
}


/// Generic function for updating a row for a given table with a given changeset.
#[inline(always)]
pub fn update_row<'a, Model, Chg, Tab>(table: Tab, changeset: Chg, conn: &PgConnection) -> Result<Model, Error>
where
    Chg: AsChangeset<Target=<Tab as HasTable>::Table>,
    Tab: QuerySource + IntoUpdateTarget,
    Update<Tab, Chg>: LoadQuery<PgConnection, Model>,
    Model: TypeName
{
    diesel::update(table)
        .set(changeset)
        .get_result::<Model>(conn)
        .map_err(handle_err::<Model>)
}




/// Generic function for creating a row for a given table with a given "new" struct for that row type.
#[inline(always)]
pub fn create_row<Model, NewModel, Tab>(table: Tab, insert: NewModel, conn: &PgConnection) -> Result<Model, Error>
where
    NewModel: Insertable<Tab>,
    InsertStatement<Tab, NewModel>: AsQuery,
    Pg: HasSqlType<<InsertStatement<Tab, NewModel> as AsQuery>::SqlType>,
    InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values>: AsQuery,
    Model: Queryable<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType, Pg>,
    Pg: HasSqlType<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType>,
    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryId,
    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryFragment<Pg>,
    Model: TypeName
{
    insert.insert_into(table)
        .get_result::<Model>(conn)
        .map_err(handle_err::<Model>)
}




//fn row_exists<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> Result<bool, Error>
//where
//    Tab: FindDsl<Uuid> + Table,
//    <Tab as FindDsl<uuid::Uuid>>::Output: SelectQuery,
//    <Tab as FindDsl<uuid::Uuid>>::Output: QueryId,
//    <Tab as FindDsl<uuid::Uuid>>::Output: ValidSubselect<()>,
//    <Tab as FindDsl<Uuid>>::Output : SelectDsl<Exists<<Tab as FindDsl<Uuid>>::Output>>,
//    Find<Tab, Uuid>: LoadQuery<PgConnection, Model>,
////    SelectStatement<(), Select=Exists<<Table as FindDsl<Uuid>>::Output>>: QueryFragment<Pg>//, SelectClause<Exists<<Table as FindDsl<uuid::Uuid>>::Output>>>: QueryFragment<Pg>
//{
////    table.find(uuid)
////        .select(exists)
//
////    select(exists(table.filter(table.primary_key().eq(uuid))))
//    table
////        .find(uuid)
//        .count()
//        .get_result::<i32>(conn)
//}
//
//use diesel::QueryDsl;
