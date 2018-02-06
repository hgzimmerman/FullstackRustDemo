use rocket_contrib::Json;


#[derive(Clone, Debug, Serialize)]
struct Topic {
    name: String
}

#[post("/topics")]
fn get_topics() -> Json<Vec<Topic>> {
    unimplemented!()
}

#[derive(Clone, Debug, Serialize)]
struct ThreadTitle {
    title: String,
    posts: usize,
    poster: String // Username?
}


#[post("/threads", data = "<topic>", rank=0)]
fn get_threads(topic: String) -> Json<Vec<ThreadTitle>> {
    unimplemented!()
}