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
use diesel::query_builder::AsQuery;
use diesel::delete;
use diesel::query_builder::DeleteStatement;
use diesel::sql_types::HasSqlType;
use diesel::insert_into;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::load_dsl::ExecuteDsl;
use diesel::query_builder::Query;
use diesel::expression::SelectableExpression;
use diesel::expression::NonAggregate;
use diesel::query_builder::AsChangeset;
use diesel::query_source::QuerySource;
use diesel::query_dsl::filter_dsl::FilterDsl;
use std::fmt::Debug;
use crate::calls::user::NewUser;
use diesel::query_builder::UpdateStatement;
use diesel::helper_types::Update;
use crate::calls::article::ArticleChangeset;
use crate::Article;


/// Generic function for getting a whole row from a given table.
#[inline(always)]
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


/// Generic function for deleting a row from a given table.
#[inline(always)]
pub fn delete_row<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> Result<Model, Error>
    where

        Tab: FindDsl<Uuid> + Table ,
        <Tab as FindDsl<Uuid>>::Output: IntoUpdateTarget,
        Pg: HasSqlType<<<<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns as Expression>::SqlType>,
        <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryId,
        <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryFragment<Pg>,
        DeleteStatement<<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table, <<Tab as FindDsl<Uuid>>::Output as IntoUpdateTarget>::WhereClause>: LoadQuery<PgConnection, Model>,
{
    delete(table.find(uuid))
        .get_result::<Model>(conn)
}

fn delete_user(uuid: UserUuid, conn: &PgConnection) -> Result<User, Error> {
    delete_row::<User,_>(schema::users::table, uuid.0, conn)
}


/// Generic function for updating a row for a given table with a given changeset.
#[inline(always)]
fn update_row<'a, Model, Chg, Tab>(table: Tab, changeset: Chg, conn: &PgConnection) -> Result<Model, Error>
where
    Chg: AsChangeset<Target=<Tab as HasTable>::Table>,
    Tab: QuerySource + IntoUpdateTarget,
    Update<Tab, Chg>: LoadQuery<PgConnection, Model>
{
    diesel::update(table)
        .set(changeset)
        .get_result::<Model>(conn)
}

fn update_article(changeset: ArticleChangeset, conn: &PgConnection) -> Result<Article, Error> {
    update_row::<Article, ArticleChangeset,_>(schema::articles::table, changeset, conn)
}


/// Generic function for creating a row for a given table with a given "new" struct for that row type.
#[inline(always)]
fn create_row<Model, NewModel, Tab>(table: Tab, insert: NewModel, conn: &PgConnection) -> Result<Model, Error>
where
    NewModel: Insertable<Tab>,
    InsertStatement<Tab, NewModel>: AsQuery,
    Pg: HasSqlType<<InsertStatement<Tab, NewModel> as AsQuery>::SqlType>,
    InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values>: AsQuery,
    Model: Queryable<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType, Pg>,
    Pg: HasSqlType<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType>,
    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryId,
    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryFragment<Pg>
{
    insert.insert_into(table)
        .get_result::<Model>(conn)
}


fn create_user(new_user: NewUser, conn: &PgConnection) -> Result<User, Error> {
    create_row::<User, NewUser,_>(schema::users::table, new_user, conn)
}