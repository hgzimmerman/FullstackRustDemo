use rocket::Route;
use rocket_contrib::Json;
use super::Routable;
use db::Conn;

use rocket::response::status::Custom;
use rocket::http::Status;
use db::article::*;
use requests_and_responses::article::*;
// use routes::DatabaseError;
use rocket::response::status::NoContent;
use error::WeekendAtJoesError;

// TODO: change the return type of this to Result<Json<Article>, Custom<>>
// return a custom 404 or a custom 500 depending on the error type
#[get("/<article_id>", rank=0)]
fn get_article(article_id: i32, conn: Conn) -> Option<Json<Article>> {
    
    match Article::get_article_by_id(article_id, &conn) {
        Ok(article_option) => article_option.and_then(|article| Some(Json(article))),
        Err(e) => {
            warn!("Getting article failed for reason: {:?}", e);
            None
        }
    }
}

#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticleRequest>, conn: Conn) -> Result<Json<Article>, Custom<&'static str>> {

    match Article::create_article(new_article.into_inner(), &conn) {
        Ok(article) => (Ok(Json(article))),
        Err(_) => Err(Custom(Status::InternalServerError, "DB Error"))
    }
}

#[put("/", data = "<update_article_request>")]
fn update_article(update_article_request: Json<UpdateArticleRequest>, conn: Conn) -> Result<Json<Article>, WeekendAtJoesError> {
    let update_article = update_article_request.into_inner();
    Article::update_article(update_article.into(), &conn).and_then(|x| Ok(Json(x)))
}

// TODO, test this interface
#[delete("/<article_id>")]
fn delete_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::delete_article(article_id, &conn) 
}

// TODO, test this interface
#[put("/publish/<article_id>")]
fn publish_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::publish_article(article_id, &conn)
}


impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![create_article, update_article, get_article, delete_article, publish_article];
    const PATH: &'static str = "/article/";
}
