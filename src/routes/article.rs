use rocket::Route;
use rocket_contrib::Json;
use uuid::Uuid;
use rocket::Rocket;
use super::Routable;
use schema::articles;
use diesel;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use rocket::State;
//use db::DB;
use diesel::PgConnection;
use std::sync::Mutex;
use db::DbConn;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
//    publish_date: String,
//    author: String, // uuid of author
    pub body: String,
    pub published: bool
}

impl Article {

}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="articles"]
struct NewArticle {
    title: String,
    body: String,
//    author: String
}


#[get("/<article_id>", rank=0)]
fn get_article(article_id: i32) -> Json<Article> {
    Json(Article {
        title: String::from("This is a title"),
//        publish_date: String::from("Today"),
        body: String::from("This is the body"),
//        author: String::from("aoeu-aoeu-aoeu-aoeu-aoeu"),
        id: article_id,
        published: false
    })
}

#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticle>, db_conn: State<DbConn>) -> Json<Article> {
    use schema::articles;
    use schema::articles::dsl::*;

//    let connection: &PgConnection = &pool.

    let conn = db_conn.inner().lock().expect("Couldn't get mutex lock on db connection");

    let new_article: NewArticle = new_article.into_inner();

    let inserted_article: Article = diesel::insert_into(articles::table)
        .values(&new_article)
        .get_result(&conn as &PgConnection)
        .expect("Failed to insert");
    
    Json(inserted_article)
}

#[put("/", data = "<article>")]
fn update_article(article: Json<Article>) -> Json<Article> {
    let article: Article = article.into_inner();
    Json(article)
}

#[delete("/<article_id>")]
fn delete_article(article_id: i32) -> Json<Article> {
    Json(Article {
        title: String::from("test"),
//        publish_date: String::from("Today"),
        body: String::from("password"),
//        author: String::from("aoeu-aoeu-aoeu-aoeu-aoeu"),
        id: article_id,
        published: false
    })
}

// Export the ROUTES and their path
pub fn article_routes() -> Vec<Route> {
    routes![create_article, update_article, get_article, delete_article]
}


impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![create_article, update_article, get_article, delete_article];
    const PATH: &'static str = "/article/";
}
