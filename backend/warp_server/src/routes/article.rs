use crate::db_integration::db_filter;
use crate::uuid_integration::uuid_filter;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use db::Conn;
use uuid::Uuid;
use identifiers::article::ArticleUuid;
use wire::article::FullArticleResponse;
use crate::error::Error;
use db::Article;
use db::article::ArticleData;
use wire::article::ArticlePreviewResponse;
use crate::jwt::normal_user_filter;
use identifiers::user::UserUuid;
use wire::article::MinimalArticleResponse;
use crate::json_body_filter;
use wire::article::NewArticleRequest;
use db::CreatableUuid;
use db::article::NewArticle;
//use db::article::ArticleChangeset;
use db::RetrievableUuid;
use wire::article::UpdateArticleRequest;

pub fn article_api() -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching Article API");
    warp::path("article")
        .and(
            get_article()
                .or(create_article())
                .or(update_article())
                .or(get_published_articles())
                .or(get_owned_unpublished_articles())
                .or(publish())
                .or(unpublish())
        )
        .with(warp::log("article"))
        .boxed()
}



fn get_article() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(uuid_filter())
        .and(db_filter())
        .and_then(|uuid: Uuid, conn: Conn| {
            let article_uuid = ArticleUuid(uuid);
            Article::get_article_data(article_uuid, &conn)
                .map(crate::convert_and_json::<ArticleData,FullArticleResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn get_published_articles() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path::param::<i32>())
        .and(warp::path::param::<i32>())
        .and(db_filter())
        .and_then(|index: i32, page_size: i32, conn: Conn| {
            Article::get_paginated(index, page_size, &conn)
                .map(crate::convert_vector_and_json::<ArticleData,ArticlePreviewResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}


fn get_owned_unpublished_articles() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path("owned_unpublished"))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|user_uuid: UserUuid, conn: Conn| {
            Article::get_unpublished_articles_for_user(user_uuid, &conn)
                .map(crate::convert_vector_and_json::<Article,MinimalArticleResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn create_article() -> BoxedFilter<(impl Reply,)> {
    warp::post2()
        .and(json_body_filter(128)) // Allow large articles
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: NewArticleRequest, user_uuid: UserUuid, conn: Conn| {
            let mut request: NewArticle = request.into();
            request.author_uuid = user_uuid.0; // This api isn't perfect - so the uuid must be gotten from the jwt

            Article::create(request.into(), &conn)
                .map(crate::convert_and_json::<Article,MinimalArticleResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}


fn update_article() -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(json_body_filter(128))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: UpdateArticleRequest, user_uuid: UserUuid, conn: Conn|{
            let article_to_update: Article = Article::get_by_uuid(request.uuid.0, &conn)
                .map_err(Error::convert_and_reject)?;
            if article_to_update.author_uuid != user_uuid.0 {
                return Error::NotAuthorized.reject();
            }

            Article::update_article(request.into(), &conn)
                .map(crate::convert_and_json::<Article,MinimalArticleResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}


fn publish() -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(warp::path("publish"))
        .and(uuid_filter())
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|uuid: Uuid, user_uuid: UserUuid, conn: Conn|{
            let article_to_update: Article = Article::get_by_uuid(uuid, &conn)
                .map_err(Error::convert_and_reject)?;
            if article_to_update.author_uuid != user_uuid.0 {
                return Error::NotAuthorized.reject()
            }

            Article::set_publish_status(ArticleUuid(uuid), true, &conn)
                .map(|_| warp::http::StatusCode::NO_CONTENT)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}
fn unpublish() -> BoxedFilter<(impl Reply,)> {
    warp::put2()
        .and(warp::path("unpublish"))
        .and(uuid_filter())
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|uuid: Uuid, user_uuid: UserUuid, conn: Conn|{
            let article_to_update: Article = Article::get_by_uuid(uuid, &conn)
                .map_err(Error::convert_and_reject)?;
            if article_to_update.author_uuid != user_uuid.0 {
                return Error::NotAuthorized.reject()
            }

            Article::set_publish_status(ArticleUuid(uuid), false, &conn)
                .map(|_| warp::http::StatusCode::NO_CONTENT)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}
