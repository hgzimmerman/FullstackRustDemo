
mod article;
pub use self::article::Article;

use views::loadable::Loadable;

pub struct NewsModel {
    pub link_id: String , // Needed???
    pub article: Loadable<Article>
}