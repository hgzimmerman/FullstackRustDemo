use schema::forums;
use db::Conn;
use std::ops::Deref;
// use diesel::RunQueryDsl;
use error::JoeResult;

#[derive(Debug, Clone, Identifiable, Queryable, Crd, ErrorHandler)]
#[insertable = "NewForum"]
#[table_name = "forums"]
pub struct Forum {
    /// Primary Key.
    pub id: i32,
    /// Displayed title of the forum
    pub title: String,
    /// The description that informs users what topics should be discussed in the forum.
    pub description: String,
}

#[derive(Insertable, Debug)]
#[table_name = "forums"]
pub struct NewForum {
    pub title: String,
    pub description: String,
}
