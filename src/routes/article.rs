use rocket::Route;
use rocket_contrib::Json;
use uuid::Uuid;
use rocket::Rocket;
use super::Routable;


#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    title: String,
    body: String,
    author: String, // uuid of author
    id: String // Uuid
}

impl Article {
    fn new(title: String, body: String, author: String ) -> Article {
        Article {
            title: title,
            body: body,
            author: author,
            id: Uuid::new_v4().hyphenated().to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewArticle {
    title: String,
    body: String,
    author: String
}


#[get("/<article_id>")]
fn get_article(article_id: String) -> Json<Article> {
    Json(Article {
        title: String::from("This is a title"),
        body: String::from("This is the body"),
        author: String::from("aoeu-aoeu-aoeu-aoeu-aoeu"),
        id: article_id,
    })
}

#[post("/", data = "<new_article>")]
fn create_article(new_article: Json<NewArticle>) -> Json<Article> {
    let new_article: NewArticle = new_article.into_inner();
    Json(Article::new(new_article.title, new_article.body, new_article.author))
}

#[put("/", data = "<article>")]
fn update_article(article: Json<Article>) -> Json<Article> {
    let article: Article = article.into_inner();
    Json(article)
}

#[delete("/<article_id>")]
fn delete_article(article_id: String) -> Json<Article> {
    Json(Article {
        title: String::from("test"),
        body: String::from("password"),
        author: String::from("aoeu-aoeu-aoeu-aoeu-aoeu"),
        id: article_id,
    })
}

// Export the ROUTES and their path
pub fn article_routes() -> Vec<Route> {
    routes![create_article, update_article, get_article, delete_article]
}


impl Routable for Article {
    const ROUTES: &'static Fn() -> Vec<Route> = &||routes![create_article, update_article, get_article, delete_article];
    const PATH: &'static str = "/article/";
}
