use crate::article::*;
use wire::article::*;

use slug;
use rand;
use rand::Rng;
use identifiers::article::ArticleUuid;
use identifiers::user::UserUuid;

impl From<Article> for MinimalArticleResponse {
    fn from(article: Article) -> MinimalArticleResponse {
        MinimalArticleResponse {
            uuid: ArticleUuid(article.uuid),
            author_uuid: UserUuid(article.author_uuid),
            title: article.title,
            body: article.body,
            publish_date: article.publish_date,
        }
    }
}

impl From<ArticleData> for FullArticleResponse {
    fn from(data: ArticleData) -> FullArticleResponse {
        FullArticleResponse {
            id: ArticleUuid(data.article.uuid),
            author: data.user.into(),
            title: data.article.title,
            body: data.article.body,
            publish_date: data.article.publish_date,
        }
    }
}

impl From<ArticleData> for ArticlePreviewResponse {
    fn from(data: ArticleData) -> ArticlePreviewResponse {
        ArticlePreviewResponse {
            uuid: ArticleUuid(data.article.uuid),
            author: data.user.into(),
            title: data.article.title,
            publish_date: data.article.publish_date,
        }
    }
}

impl From<UpdateArticleRequest> for ArticleChangeset {
    fn from(request: UpdateArticleRequest) -> ArticleChangeset {
        ArticleChangeset {
            uuid: request.uuid.0,
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
            author_uuid: new_article_request.author_id.0,
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
