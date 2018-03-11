use rocket::Route;
use rocket_contrib::Json;
use super::{Routable, convert_vector};
use db::Conn;
use db::Retrievable;
use db::Creatable;

use db::article::*;
use requests_and_responses::article::*;
use rocket::response::status::NoContent;
use error::WeekendAtJoesError;
use auth::user_authorization::NormalUser;


/// Gets an article by id.
#[get("/<article_id>", rank = 0)]
fn get_article(article_id: i32, conn: Conn) -> Result<Json<ArticleResponse>, WeekendAtJoesError> {
    Article::get_by_id(article_id, &conn)
        .map(ArticleResponse::from)
        .map(Json)
}


#[get("/articles/<number_of_articles>", rank = 0)]
fn get_published_articles(number_of_articles: i64, conn: Conn) -> Result<Json<Vec<ArticleResponse>>, WeekendAtJoesError> {
    Article::get_recent_published_articles(number_of_articles, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Gets the articles that haven't been published yet that are associated with the logged in user.
#[get("/users_unpublished_articles")]
fn get_users_unpublished_articles(logged_in_user: NormalUser, conn: Conn) -> Result<Json<Vec<ArticleResponse>>, WeekendAtJoesError> {
    let name = logged_in_user.user_name; // extract the username from the jwt
    Article::get_unpublished_articles_for_username(name, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Creates a new article.
/// The user id of the user must match the author id of the article being created.
#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticleRequest>, user: NormalUser, conn: Conn) -> Result<Json<ArticleResponse>, WeekendAtJoesError> {
    if new_article.author_id != user.user_id {
        return Err(WeekendAtJoesError::NotAuthorized {
            reason: "Article being created's user does not match the user's id.",
        });
    }

    Article::create(new_article.into_inner().into(), &conn)
        .map(ArticleResponse::from)
        .map(Json)
}

/// Performs an update on an article.
/// The user id of the user must match the article being updated.
#[put("/", data = "<update_article_request>")]
fn update_article(update_article_request: Json<UpdateArticleRequest>, user: NormalUser, conn: Conn) -> Result<Json<ArticleResponse>, WeekendAtJoesError> {
    let article_to_update: Article = Article::get_by_id(update_article_request.id, &conn)?;
    if article_to_update.author_id != user.user_id {
        return Err(WeekendAtJoesError::NotAuthorized { reason: "Article being updated does not match the user's id." });
    }

    let update_article = update_article_request.into_inner();
    Article::update_article(update_article.into(), &conn)
        .map(ArticleResponse::from)
        .map(Json)
}


/// Given an article id, set the corresponding article's date_published column to contain the current date.
#[put("/publish/<article_id>")]
fn publish_article(article_id: i32, user: NormalUser, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    let article_to_update: Article = Article::get_by_id(article_id, &conn)?;
    if article_to_update.author_id != user.user_id {
        return Err(WeekendAtJoesError::NotAuthorized { reason: "Article being updated does not match the user's id." });
    }

    Article::set_publish_status(article_id, true, &conn)
        .map(|_| NoContent)
}

/// Given an article id, set the corresponding article's date_published colum to NULL.
#[put("/unpublish/<article_id>")]
fn unpublish_article(article_id: i32, user: NormalUser, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    let article_to_update: Article = Article::get_by_id(article_id, &conn)?;
    if article_to_update.author_id != user.user_id {
        return Err(WeekendAtJoesError::NotAuthorized { reason: "Article being updated does not match the user's id." });
    }

    Article::set_publish_status(article_id, false, &conn)
        .map(|_| NoContent)
}

impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_article,
            update_article,
            get_article,
            get_published_articles,
            get_users_unpublished_articles,
            // delete_article
            publish_article,
            unpublish_article,
        ]
    };
    const PATH: &'static str = "/article/";
}
