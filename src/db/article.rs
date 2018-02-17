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

/// The database's representation of an article
#[derive(Serialize, Deserialize, Clone, Queryable, Identifiable, Associations, Debug, PartialEq)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name="articles"]
pub struct Article {
    /// The public key for the article.
    pub id: i32,
    /// The key of the user that authored the article.
    pub author_id: i32,
    /// The title of the article. This will be used to when showing the article in "Suggested Articles" panes.
    pub title: String,
    /// The body will be rendered in markdown and will constitute the main content of the article.
    pub body: String,
    /// The presense of a publish date will idicate the article's published status,
    /// and will be used in ordering sets of the most recent articles.
    pub publish_date: Option<NaiveDateTime>
}

/// Specifies the attributes that can be changed for an article.
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
    /// Gets the article associated with the id.
    pub fn get_article_by_id(article_id: i32, conn: &Conn) -> Result<Article, WeekendAtJoesError> {
        use schema::articles::dsl::*;

        articles
            .find(article_id)
            .first(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Article"))
    }

    /// Gets the n most recent articles, where n is specified by the number_of_articles parameter.
    /// The the returned articles will only include ones with a publish date.
    pub fn get_recent_published_articles(number_of_articles: i64, conn: &Conn) -> Result<Vec<Article>, WeekendAtJoesError> {
        use schema::articles::dsl::*;

        let returned_articles: Result<Vec<Article>, Error> = articles
            .filter(publish_date.is_not_null())
            .limit(number_of_articles)
            .order(publish_date)
            .load::<Article>(conn.deref());
        
        returned_articles.or(Err(WeekendAtJoesError::DatabaseError(None)))
    }

    /// Gets the unpublished articles for a given user 
    // TODO, consiter switching this interface to take a user_id instead of a string
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

    // Creates a new article
    pub fn create_article(new_article_request: NewArticleRequest, conn: &Conn) -> Result<Article, WeekendAtJoesError> {
        use schema::articles;

        let new_article: NewArticle = new_article_request.into();

        diesel::insert_into(articles::table)
            .values(&new_article)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Article"))
            
    }

    /// Marks the article as published, this allows the article to be viewed people other than the author.
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

    /// Hide the article from public view by setting its published date to NULL.
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

    /// Applies the changeset to its corresponding article.
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
    
    /// Deletes the article corresponding to the provided id
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

/// Represents an article that will be inserted into the database.
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