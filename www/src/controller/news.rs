use views::loadable::Loadable;
use controller::{Msg, Updatable};
use yew::html::Context;
use yew::services::format::{Nothing, Json};
use yew::services::fetch::{FetchService, Method};

use models::Article;
use models::NewsModel;
use controller::format_url;

pub enum NewsMsg {
    FetchArticle {
        id: String
    },
    ArticleReady(Result<Article, ()>)
}


impl Updatable<NewsMsg> for NewsModel {
    fn update(&mut self, context: &mut Context<Msg>, msg: NewsMsg) {
        use self::NewsMsg::*;
        match msg {
            FetchArticle { id } => {
                self.article = Loadable::Loading;
                let route = format!("/api/article/{}", id); // TODO possible use the std::path to validate this properly
                context.fetch(Method::Get, route.as_str(), Nothing, |Json(data)| {
                    Msg::News(ArticleReady(data))
                });
            }
            ArticleReady(article) => {
                match article {
                    Ok(a) => {
                        self.article = Loadable::Loaded(a);
                    }
                    Err(e) => {
                        self.article = Loadable::Unloaded;
                    }
                }
            }
        }
    }
}