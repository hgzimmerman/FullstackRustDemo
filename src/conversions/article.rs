use db::article::*;
use requests_and_responses::article::*;

use slug;
use rand;
use rand::Rng;

impl From<Article> for ArticleResponse {
    fn from(article: Article) -> ArticleResponse {
        ArticleResponse {
            id: article.id,
            author_id: article.author_id,
            title: article.title,
            body: article.body,
            publish_date: article.publish_date,
        }
    }
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
