use schema::articles;
use diesel::result::Error;
use std::ops::Deref;
use db::Conn;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::Insertable;
use routes::DatabaseError;
use rocket::response::status::NoContent;
use requests_and_responses::article::NewArticleRequest;
use routes::WeekendAtJoesError;

#[derive(Serialize, Deserialize, Clone, Queryable, AsChangeset, Identifiable, Associations, Debug, PartialEq)]
#[belongs_to(User)]
#[table_name="articles"]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
//    publish_date: String,
//    author: Uuid, // uuid of author
    pub body: String,
    pub published: bool
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

    pub fn create_article(new_article_request: NewArticleRequest, conn: &Conn) -> Result<Article, Error> {
        use schema::articles;

        let new_article: NewArticle = new_article_request.into();

        diesel::insert_into(articles::table)
            .values(&new_article)
            .get_result(conn.deref())
    }

    pub fn publish_article(article_id: i32, conn: &Conn) -> Result<Option<NoContent>, DatabaseError> {
        use schema::articles::dsl::*;
        use schema::articles;
        match diesel::update(articles::table)
            .filter(id.eq(article_id))
            .set(published.eq(true))
            .execute(conn.deref()) 
        {
            Ok(_) => Ok(Some(NoContent)),
            Err(e) => match e {
                Error::NotFound => Ok(None),
                _ => Err(DatabaseError(None))
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