use schema::forums;
use uuid::Uuid;

#[derive(Debug, Clone, Identifiable, Queryable, CrdUuid, ErrorHandler)]
#[primary_key(uuid)]
#[insertable = "NewForum"]
#[table_name = "forums"]
pub struct Forum {
    /// Primary Key.
    pub uuid: Uuid,
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
