use schema::articles;
use diesel::result::Error;
use std::ops::Deref;
use db::Conn;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use rocket::response::status::NoContent;
use requests_and_responses::article::NewArticleRequest;
use error::WeekendAtJoesError;
use chrono::{NaiveDateTime, Utc};
use requests_and_responses::article::*;

#[derive(Serialize, Deserialize, Clone, Queryable, Identifiable, Associations, Debug, PartialEq)]
#[belongs_to(User)]
#[table_name="articles"]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
//    publish_date: String,
//    author: Uuid, // uuid of author
    pub body: String,
    pub publish_date: Option<NaiveDateTime>
}

#[derive(AsChangeset, Clone, PartialEq)]
#[table_name="articles"]
pub struct ArticleChangeset {
    id: i32,
    title: Option<String>,
    body: Option<String>,
}

impl From<UpdateArticleRequest> for ArticleChangeset {
    fn from(request: UpdateArticleRequest) -> ArticleChangeset {
        ArticleChangeset {
            id: request.id,
            title: request.title,
            body: request.body,
        }
    }
}

impl Article {
    pub fn get_article_by_id(article_id: i32, conn: &Conn) -> Result<Option<Article>, Error> {
        use schema::articles::dsl::*;

        let returned_articles: Result<Vec<Article>, Error> = articles
            .filter(id.eq(article_id))
            .limit(1)
            .load::<Article>(conn.deref());
        
        returned_articles.and_then(|x| Ok(x.get(0).cloned()))
    }

    pub fn get_recent_published_articles(number_of_articles: i64, conn: &Conn) -> Result<Vec<Article>, WeekendAtJoesError> {
        use schema::articles::dsl::*;

        let returned_articles: Result<Vec<Article>, Error> = articles
            .filter(publish_date.is_not_null())
            .limit(number_of_articles)
            .order(publish_date)
            .load::<Article>(conn.deref());
        
        returned_articles.or(Err(WeekendAtJoesError::DatabaseError(None)))
    }


    pub fn create_article(new_article_request: NewArticleRequest, conn: &Conn) -> Result<Article, Error> {
        use schema::articles;

        let new_article: NewArticle = new_article_request.into();

        diesel::insert_into(articles::table)
            .values(&new_article)
            .get_result(conn.deref())
    }

    pub fn publish_article(article_id: i32, conn: &Conn) -> Result<NoContent, WeekendAtJoesError> {
        use schema::articles::dsl::*;
        use schema::articles;
        match diesel::update(articles::table)
            .filter(id.eq(article_id))
            .set(publish_date.eq(Utc::now().naive_utc()))
            .execute(conn.deref()) 
        {
            Ok(_) => Ok(NoContent),
            Err(e) => match e {
                Error::NotFound => Err(WeekendAtJoesError::NotFound{type_name: "Article"}),
                _ => Err(WeekendAtJoesError::DatabaseError(None))
            }
        }
    }

    pub fn unpublish_article(article_id: i32, conn: &Conn) -> Result<NoContent, WeekendAtJoesError> {
        use schema::articles::dsl::*;
        use schema::articles;
        match diesel::update(articles::table)
            .filter(id.eq(article_id))
            .set(publish_date.eq(None as Option<NaiveDateTime>))
            .execute(conn.deref()) 
        {
            Ok(_) => Ok(NoContent),
            Err(e) => match e {
                Error::NotFound => Err(WeekendAtJoesError::NotFound{type_name: "Article"}),
                _ => Err(WeekendAtJoesError::DatabaseError(None))
            }
        }
    }

    pub fn update_article(changeset: ArticleChangeset, conn: &Conn) -> Result<Article, WeekendAtJoesError> {
        use schema::articles;
        match diesel::update(articles::table)
            // .filter(id.eq(article_id))
            .set(&changeset)
            .get_result(conn.deref()) 
        {
            Ok(article) => Ok(article),
            Err(e) => match e {
                Error::NotFound => Err(WeekendAtJoesError::NotFound{type_name: "Article"}),
                _ => Err(WeekendAtJoesError::DatabaseError(None))
            }
        }
    }

    pub fn delete_article(article_id: i32, conn: &Conn) -> Result<NoContent, WeekendAtJoesError> {
        use schema::articles::dsl::*;

        match diesel::delete(articles.filter(id.eq(article_id)))
            .execute(conn.deref())
        {
            Ok(_) => Ok(NoContent),
            Err(e) => match e {
                Error::NotFound => Err(WeekendAtJoesError::NotFound{type_name: "Article"}),
                _ => Err(WeekendAtJoesError::DatabaseError(None))
            }
        }
    }
    
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="articles"]
pub struct NewArticle {
    pub title: String,
    pub body: String,
    pub author_id: i32
}

impl From<NewArticleRequest> for NewArticle {
    fn from(new_article_request: NewArticleRequest) -> NewArticle {
        NewArticle {
            title: new_article_request.title,
            body: new_article_request.body,
            author_id: new_article_request.author_id
        }
    }
}