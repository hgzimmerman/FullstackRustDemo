mod article;
pub use self::article::Article;

use yew::html::Href;
use views::loadable::Loadable;

pub struct Model {
    pub page: Page
}

pub enum Page {
    News(NewsModel),
    BucketQuestions,
}

impl<'a> Into<Href> for &'a Page {
    fn into(self) -> Href {
        match *self {
            Page::News(_) => "#/news".into(),
            Page::BucketQuestions => "#/bucket_questions".into(),
        }
    }
}




pub struct NewsModel {
    pub link_id: String , // Needed???
    pub article: Loadable<Article>
}

