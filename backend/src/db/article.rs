use schema::articles;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use chrono::{NaiveDateTime, Utc};
use db::user::User;
use diesel::BelongingToDsl;
use error::JoeResult;
use diesel::PgConnection;
use uuid::Uuid;
use identifiers::article::ArticleUuid;
use identifiers::user::UserUuid;


/// The database's representation of an article
#[derive(Clone, Queryable, Identifiable, Associations, CrdUuid, ErrorHandler, Debug, PartialEq)]
#[insertable = "NewArticle"]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "articles"]
pub struct Article {
    /// The public key for the article.
    pub id: Uuid,
    /// The key of the user that authored the article.
    pub author_id: Uuid,
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
#[derive(AsChangeset, Clone, Debug, PartialEq)]
#[table_name = "articles"]
pub struct ArticleChangeset {
    pub id: Uuid,
    pub title: Option<String>,
    pub body: Option<String>,
}



/// Represents an article that will be inserted into the database.
#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub body: String,
    pub author_id: Uuid,
}

pub struct ArticleData {
    pub article: Article,
    pub user: User,
}


impl Article {
    // /// Gets the n most recent articles, where n is specified by the number_of_articles parameter.
    // /// The the returned articles will only include ones with a publish date.
    // pub fn get_recent_published_articles(number_of_articles: i64, conn: &Conn) -> JoeResult<Vec<Article>> {
    //     use schema::articles::dsl::*;

    //     let returned_articles: Result<Vec<Article>, Error> = articles
    //         .filter(publish_date.is_not_null())
    //         .limit(number_of_articles)
    //         .order(publish_date)
    //         .load::<Article>(conn.deref());

    //     returned_articles.or(Err(
    //         WeekendAtJoesError::DatabaseError(None),
    //     ))
    // }

    pub fn get_article_data(article_uuid: ArticleUuid, conn: &PgConnection) -> JoeResult<ArticleData> {

        let article = Article::get_by_uuid(article_uuid.0, conn)?;
        let user = User::get_by_uuid(article.author_id, conn)?;
        Ok(ArticleData { article, user })
    }

    pub fn get_paginated(page_index: i32, page_size: i32, conn: &PgConnection) -> JoeResult<Vec<ArticleData>> {
        use schema::articles::dsl::*;
        use db::diesel_extensions::pagination::*;
        use schema::users;

        let (articles_and_users, _count) = articles
            .inner_join(users::table)
            .filter(publish_date.is_not_null())
            .order(publish_date)
            .paginate(page_index.into())
            .per_page(page_size.into())
            .load_and_count_pages::<(Article, User)>(conn)
            .map_err(Article::handle_error)?;

        let article_data = articles_and_users
            .into_iter()
            .map(|x| {
                ArticleData {
                    article: x.0,
                    user: x.1,
                }
            })
            .collect();

        Ok(article_data)
    }




    /// Gets the unpublished articles for a given user
    pub fn get_unpublished_articles_for_user(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<Vec<Article>> {
        use schema::articles::dsl::*;
        use schema::users::dsl::*;
//        use schema::users;

        let user: User = users
            .find(user_uuid.0)
            .get_result::<User>(conn)
            .map_err(User::handle_error)?;


        Article::belonging_to(&user)
            .filter(publish_date.is_null())
            .order(publish_date)
            .load::<Article>(conn)
            .map_err(Article::handle_error)

    }

    /// Sets the date for the article's publish date.
    /// If true, it will set the publish datetime to the current time, indicating it is published.
    /// If false, it will set the publish column to Null, indicating that it has not been published.
    pub fn set_publish_status(article_uuid: ArticleUuid, publish: bool, conn: &PgConnection) -> JoeResult<Article> {
        use schema::articles::dsl::*;
        use schema::articles;

        let publish_value: Option<NaiveDateTime> = if publish {
            Some(Utc::now().naive_utc())
        } else {
            None
        };

        diesel::update(articles::table)
            .filter(articles::id.eq(article_uuid.0))
            .set(publish_date.eq(publish_value))
            .get_result(conn)
            .map_err(Article::handle_error)
    }


    /// Applies the changeset to its corresponding article.
    pub fn update_article(changeset: ArticleChangeset, conn: &PgConnection) -> JoeResult<Article> {
        use schema::articles;
        diesel::update(articles::table)
            .set(&changeset)
            .get_result(conn)
            .map_err(Article::handle_error)
    }
}
