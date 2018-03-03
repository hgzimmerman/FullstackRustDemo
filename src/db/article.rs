use schema::articles;
use diesel::result::Error;
use std::ops::Deref;
use db::Conn;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use requests_and_responses::article::NewArticleRequest;
use error::*;
use chrono::{NaiveDateTime, Utc};
use requests_and_responses::article::*;
use db::user::User;
use diesel::BelongingToDsl;
use slug;
use rand;
use rand::Rng;

/// The database's representation of an article
#[derive(Clone, Queryable, Identifiable, Associations, Crd, ErrorHandler, Debug, PartialEq)]
#[insertable = "NewArticle"]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "articles"]
pub struct Article {
    /// The public key for the article.
    pub id: i32,
    /// The key of the user that authored the article.
    pub author_id: i32,
    /// The title of the article. This will be used to when showing the article in "Suggested Articles" panes.
    pub title: String,
    /// Converted title + suffix for use in urls
    pub slug: String,
    /// The body will be rendered in markdown and will constitute the main content of the article.
    pub body: String,
    /// The presense of a publish date will idicate the article's published status,
    /// and will be used in ordering sets of the most recent articles.
    pub publish_date: Option<NaiveDateTime>,
}

/// Specifies the attributes that can be changed for an article.
#[derive(AsChangeset, Clone, PartialEq)]
#[table_name = "articles"]
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

/// Represents an article that will be inserted into the database.
#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub body: String,
    pub author_id: i32,
}

impl From<NewArticleRequest> for NewArticle {
    fn from(new_article_request: NewArticleRequest) -> NewArticle {
        NewArticle {
            title: new_article_request.title.clone(),
            slug: slugify(&new_article_request.title),
            body: new_article_request.body,
            author_id: new_article_request.author_id,
        }
    }
}


impl Article {
    /// Gets the n most recent articles, where n is specified by the number_of_articles parameter.
    /// The the returned articles will only include ones with a publish date.
    pub fn get_recent_published_articles(number_of_articles: i64, conn: &Conn) -> Result<Vec<Article>, WeekendAtJoesError> {
        use schema::articles::dsl::*;

        let returned_articles: Result<Vec<Article>, Error> = articles
            .filter(publish_date.is_not_null())
            .limit(number_of_articles)
            .order(publish_date)
            .load::<Article>(conn.deref());

        returned_articles.or(Err(
            WeekendAtJoesError::DatabaseError(None),
        ))
    }

    /// Gets the unpublished articles for a given user
    // TODO, consiter switching this interface to take a user_id instead of a string
    pub fn get_unpublished_articles_for_username(username: String, conn: &Conn) -> Result<Vec<Article>, WeekendAtJoesError> {
        use schema::articles::dsl::*;
        use schema::users::dsl::*;

        let user: User = users
            .filter(user_name.eq(username))
            .get_result::<User>(conn.deref())
            .map_err(User::handle_error)?;


        Article::belonging_to(&user)
            .filter(publish_date.is_null())
            .order(publish_date)
            .load::<Article>(conn.deref())
            .map_err(Article::handle_error)

    }


    pub fn set_publish_status(article_id: i32, publish: bool, conn: &Conn) -> Result<Article, WeekendAtJoesError> {
        use schema::articles::dsl::*;
        use schema::articles;

        let publish_value: Option<NaiveDateTime> = if publish {
            Some(Utc::now().naive_utc())
        } else {
            None
        };

        diesel::update(articles::table)
            .filter(id.eq(article_id))
            .set(publish_date.eq(publish_value))
            .get_result(conn.deref())
            .map_err(Article::handle_error)
    }


    /// Applies the changeset to its corresponding article.
    pub fn update_article(changeset: ArticleChangeset, conn: &Conn) -> Result<Article, WeekendAtJoesError> {
        use schema::articles;
        diesel::update(articles::table)
            .set(&changeset)
            .get_result(conn.deref())
            .map_err(Article::handle_error)
    }
}






const SUFFIX_LEN: usize = 6;

fn slugify(title: &str) -> String {
    // if cfg!(feature = "random_suffix") {
    format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
    // } else {
    // slug::slugify(title)
    // }
}

fn generate_suffix(len: usize) -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .take(len)
        .collect::<String>()
}
