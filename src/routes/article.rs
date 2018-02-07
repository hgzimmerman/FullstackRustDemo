use rocket::Route;
use rocket_contrib::Json;
use rocket::Rocket;
use super::Routable;
use schema::articles;
use diesel;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
//use db::DB;
use db::DbConn;
use db::Conn;
use db::Pool;

#[derive(Serialize, Deserialize, Clone, Queryable, AsChangeset, Identifiable, Debug)]
#[table_name="articles"]
pub struct Article {
    pub id: i32,
    pub title: String,
//    publish_date: String,
//    author: Uuid, // uuid of author
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
fn get_article(article_id: i32, db_conn: Conn) -> Option<Json<Article>> {
    use schema::articles;
    use schema::articles::dsl::*;

    let returned_articles: Vec<Article> = articles
        .filter(id.eq(article_id))
        .limit(1)
        .load::<Article>(&*db_conn)
        .expect("db error");

        match returned_articles.get(0) {
            Some(a) => Some(Json(a.clone())),
            None => None
        }
}

#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticle>, db_conn: Conn) -> Json<Article> {
    use schema::articles;


    let new_article: NewArticle = new_article.into_inner();

    let inserted_article: Article = diesel::insert_into(articles::table)
        .values(&new_article)
        .get_result(&*db_conn)
        .expect("Failed to insert");
    
    Json(inserted_article)
}

#[put("/", data = "<update_article>")]
fn update_article(update_article: Json<Article>, db_conn: Conn) -> Json<Article> {
    use schema::articles::dsl::*;
    use schema::articles;

    let article: Article = update_article.into_inner();

    let updated_article: Article = diesel::update(articles::table)
        .set(&article)
        .get_result(&*db_conn)
        .expect("Failed to insert");
    
    Json(updated_article)
}

#[delete("/<article_id>")]
fn delete_article(article_id: i32, db_conn: Conn) -> Json<Article> {
    use schema::articles;
    use schema::articles::dsl::*;

    // let conn = db_conn.inner().lock().expect("Couldn't get mutex lock on db connection");

    let deleted_article = diesel::delete(articles.filter(id.eq(article_id)))
        .get_result(&*db_conn)
        .expect("Failed to delete");
    Json(deleted_article)
}

// Export the ROUTES and their path
pub fn article_routes() -> Vec<Route> {
    routes![create_article, update_article, get_article, delete_article]
}


impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![create_article, update_article, get_article, delete_article];
    const PATH: &'static str = "/article/";
}
