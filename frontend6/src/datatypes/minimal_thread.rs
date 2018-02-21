
#[derive(Clone, Debug, PartialEq)]
pub struct MinimalThread {
    pub id: i32,
    pub title: String,
    pub author: String,
    // pub created_date: NaiveDateTime,
    pub replies: i32,
    pub locked: bool,
}

impl Default for MinimalThread {
    fn default() -> MinimalThread {
        MinimalThread {
            id: 0,
            title: "".into(),
            author: "".into(),
            replies: 0,
            locked: false
        }
    }
}