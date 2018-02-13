use rocket::Route;
use rocket_contrib::Json;
use super::Routable;
use db::Conn;

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

#[get("/articles/<number_of_articles>", rank=0)]
fn get_published_articles(number_of_articles: i64, conn: Conn) -> Result<Json<Vec<Article>>, WeekendAtJoesError> {
    Article::get_published_articles(number_of_articles, &conn).and_then(|a| Ok(Json(a)))
}

#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticleRequest>, conn: Conn) -> Result<Json<Article>, WeekendAtJoesError> {

    match Article::create_article(new_article.into_inner(), &conn) {
        Ok(article) => (Ok(Json(article))),
        Err(_) => Err(WeekendAtJoesError::DatabaseError(None))
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

#[put("/unpublish/<article_id>")]
fn unpublish_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::unpublish_article(article_id, &conn)
}

impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![
            create_article,
            update_article,
            get_article,
            get_published_articles,
            delete_article,
            publish_article,
            unpublish_article
        ];
    const PATH: &'static str = "/article/";
}
