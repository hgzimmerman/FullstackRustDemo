use views::loadable::Loadable;
use models::Article;

pub struct NewsModel {
    pub link_id: String , // Needed???
    pub article: Loadable<Article>
}
