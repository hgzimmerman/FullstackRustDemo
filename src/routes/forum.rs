use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

pub struct Forum;

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


#[post("/<topic>/threads", rank=0)]
fn get_threads(topic: String) -> Json<Vec<ThreadTitle>> {
    unimplemented!()
}

impl Routable for Forum {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![get_topics, get_threads];
    const PATH: &'static str = "/forum/";
}

