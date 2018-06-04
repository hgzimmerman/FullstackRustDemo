use rocket::Route;
use rocket_contrib::Json;
use super::{Routable, convert_vector};
use db::Conn;

use db::article::*;
use wire::article::*;
use rocket::response::status::NoContent;
use error::WeekendAtJoesError;
use auth::user_authorization::NormalUser;
use identifiers::article::ArticleUuid;
use db::RetrievableUuid;
use db::CreatableUuid;


/// Gets an article by id.
#[get("/<article_uuid>", rank = 0)]
fn get_article(article_uuid: ArticleUuid, conn: Conn) -> Result<Json<FullArticleResponse>, WeekendAtJoesError> {
    Article::get_article_data(article_uuid, &conn)
        .map(FullArticleResponse::from)
        .map(Json)
}


/// Gets the published articles.
#[get("/articles/<index>/<page_size>", rank = 0)]
fn get_published_articles(index: i32, page_size: i32, conn: Conn) -> Result<Json<Vec<ArticlePreviewResponse>>, WeekendAtJoesError> {
    Article::get_paginated(index, page_size, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Gets the articles that haven't been published yet that are associated with the logged in user.
#[get("/users_unpublished_articles")]
fn get_users_unpublished_articles(logged_in_user: NormalUser, conn: Conn) -> Result<Json<Vec<MinimalArticleResponse>>, WeekendAtJoesError> {
    //    let name = logged_in_user.user_name; // extract the username from the jwt

    Article::get_unpublished_articles_for_user(logged_in_user.user_id, &conn)
        .map(convert_vector)
        .map(Json)
}

/// Creates a new article.
/// The user id of the user must match the author id of the article being created.
#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticleRequest>, user: NormalUser, conn: Conn) -> Result<Json<MinimalArticleResponse>, WeekendAtJoesError> {
    if new_article.author_id != user.user_id {
        return Err(WeekendAtJoesError::NotAuthorized {
            reason: "Article being created's user does not match the user's id.",
        });
    }

    Article::create(new_article.into_inner().into(), &conn)
        .map(MinimalArticleResponse::from)
        .map(Json)
}

/// Performs an update on an article.
/// The user id of the user must match the article being updated.
#[put("/", data = "<update_article_request>")]
fn update_article(update_article_request: Json<UpdateArticleRequest>, user: NormalUser, conn: Conn) -> Result<Json<MinimalArticleResponse>, WeekendAtJoesError> {
    let article_to_update: Article = Article::get_by_uuid(update_article_request.id.0, &conn)?;
    if article_to_update.author_id != user.user_id.0 {
        return Err(WeekendAtJoesError::NotAuthorized { reason: "Article being updated does not match the user's id." });
    }

    let update_article = update_article_request.into_inner();
    Article::update_article(update_article.into(), &conn)
        .map(MinimalArticleResponse::from)
        .map(Json)
}


/// Given an article id, set the corresponding article's date_published column to contain the current date.
#[put("/publish/<article_uuid>")]
fn publish_article(article_uuid: ArticleUuid, user: NormalUser, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    let article_to_update: Article = Article::get_by_uuid(article_uuid.0, &conn)?;
    if article_to_update.author_id != user.user_id.0 {
        return Err(WeekendAtJoesError::NotAuthorized { reason: "Article being updated does not match the user's id." });
    }

    Article::set_publish_status(article_uuid, true, &conn)
        .map(|_| NoContent)
}

/// Given an article id, set the corresponding article's date_published colum to NULL.
#[put("/unpublish/<article_uuid>")]
fn unpublish_article(article_uuid: ArticleUuid, user: NormalUser, conn: Conn) -> Result<NoContent, WeekendAtJoesError> {
    let article_to_update: Article = Article::get_by_uuid(article_uuid.0, &conn)?;
    if article_to_update.author_id != user.user_id.0 {
        return Err(WeekendAtJoesError::NotAuthorized { reason: "Article being updated does not match the user's id." });
    }

    Article::set_publish_status(article_uuid, false, &conn)
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
