use schema::forums;


#[derive( Debug, Clone, Identifiable, Queryable)]
#[table_name="forums"]
pub struct Forum {
    id: i32,
    title: String
}