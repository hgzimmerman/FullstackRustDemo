use rocket::Route;
use rocket_contrib::Json;
use super::Routable;
use db::Conn;

use db::article::*;
use requests_and_responses::article::*;
use rocket::response::status::NoContent;
use error::WeekendAtJoesError;
use auth::user_authorization::NormalUser;

impl From<Article> for ArticleResponse {
    fn from(article: Article) -> ArticleResponse {
        ArticleResponse {
            id: article.id,
            author_id: article.author_id,
            title: article.title,
            body: article.body,
            publish_date: article.publish_date
        }
    }
}

#[get("/<article_id>", rank=0)]
fn get_article(article_id: i32, conn: Conn) -> Result<Json<ArticleResponse>, WeekendAtJoesError> {
    Article::get_article_by_id(article_id, &conn)
        .map(|article| Json(article.into()))
}

#[get("/articles/<number_of_articles>", rank=0)]
fn get_published_articles(number_of_articles: i64, conn: Conn) -> Result<Json<Vec<ArticleResponse>>, WeekendAtJoesError> {
    Article::get_recent_published_articles(number_of_articles, &conn)
        .map(|articles| Json(articles.into_iter().map(|article| article.into()).collect()))
}

#[get("/users_unpublished_articles")]
fn get_users_unpublished_articles(logged_in_user: NormalUser, conn: Conn) -> Result<Json<Vec<ArticleResponse>>, WeekendAtJoesError> {
    let name = logged_in_user.user_name; // extract the username from the jwt
    Article::get_unpublished_articles_for_username(name, &conn)
        .map(|articles| Json(articles.into_iter().map(|article| article.into()).collect()))
}

#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticleRequest>, conn: Conn) -> Result<Json<ArticleResponse>, WeekendAtJoesError> {

    Article::create_article(new_article.into_inner(), &conn)
        .map(|a| Json(a.into()))
}

#[put("/", data = "<update_article_request>")]
fn update_article(update_article_request: Json<UpdateArticleRequest>, conn: Conn) -> Result<Json<ArticleResponse>, WeekendAtJoesError> {
    let update_article = update_article_request.into_inner();
    Article::update_article(update_article.into(), &conn)
        .map(|a| Json(a.into()))
}

// TODO, test this interface
#[delete("/<article_id>")]
fn delete_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::delete_article(article_id, &conn)
        .map(|_| NoContent)
}

// TODO, test this interface
#[put("/publish/<article_id>")]
fn publish_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::set_publish_status(article_id, true, &conn)
        .map(|_| NoContent)
}

#[put("/unpublish/<article_id>")]
fn unpublish_article(article_id: i32, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    Article::set_publish_status(article_id, false, &conn)
        .map(|_| NoContent)
}

impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![
            create_article,
            update_article,
            get_article,
            get_published_articles,
            get_users_unpublished_articles,
            delete_article,
            publish_article,
            unpublish_article
        ];
    const PATH: &'static str = "/article/";
}
