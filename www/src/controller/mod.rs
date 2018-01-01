use models::Page;
use views::loadable::Loadable;
use models::Article;
use models::{Model, NewsModel};
use yew::html::Context;


mod news;
pub use self::news::NewsMsg;

mod bucket_questions;
pub use self::bucket_questions::BucketMsg;

pub enum Msg {
    // Top level
    /// Set the page
    SetTopLevelPage(Page),
    // Defer to sub-controllers
    /// Defer to the NewsModel controller
    News(NewsMsg),
    /// Defer to the BucketQuestion controller
    BucketQuestion(BucketMsg),
    /// Perform no action.
    NoOp
}

pub fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    use Msg::*;
    match msg {
        SetTopLevelPage(page) => {
            model.page = page;
        }
        News(news_msg) => {
            if let Page::News(ref mut news_model) = model.page {
                news_model.update(context, news_msg)
            }
        }
        BucketQuestion(bucket_msg) => {
            if let Page::BucketQuestions(ref mut bucket_model) = model.page {
                bucket_model.update(context, bucket_msg)
            }
        }
        NoOp => {
            // do nothing
        }

    }
}

/// A trait used by sub-controllers.
/// A sub-controller will be responsible for controlling the updating of a specific sub-models.
pub trait Updatable<M> {
    fn update(&mut self, context: &mut Context<Msg>, msg: M);
}

pub fn format_url<'a>(route: String) -> String {
    let domain_and_port = "localhost:8001";
    format!("{domain}/{route}", domain=domain_and_port, route=route)
}