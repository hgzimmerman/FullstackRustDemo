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
use db::user::User;
use diesel::BelongingToDsl;
use db::handle_diesel_error;

#[derive(Serialize, Deserialize, Clone, Queryable, Identifiable, Associations, Debug, PartialEq)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name="articles"]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
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
    pub fn get_article_by_id(article_id: i32, conn: &Conn) -> Result<Article, WeekendAtJoesError> {
        use schema::articles::dsl::*;

        articles
            .find(article_id)
            .first(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Article"))
        
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

    pub fn get_unpublished_articles_for_username(username: String, conn: &Conn) -> Result<Vec<Article>, WeekendAtJoesError> {
        use schema::articles::dsl::*;
        use schema::users::dsl::*;

        let user: User = users
            .filter(user_name.eq(username))
            .get_result::<User>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))?;


        Article::belonging_to(&user)
            .filter(
                publish_date.is_null(),
            )
            .order(publish_date)
            .load::<Article>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Article"))
        
    }

    
    pub fn create_article(new_article_request: NewArticleRequest, conn: &Conn) -> Result<Article, WeekendAtJoesError> {
        use schema::articles;

        let new_article: NewArticle = new_article_request.into();

        diesel::insert_into(articles::table)
            .values(&new_article)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Article"))
            
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