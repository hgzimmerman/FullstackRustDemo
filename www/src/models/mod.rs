
mod news;
pub use self::news::*;

mod bucket;
pub use self::bucket::*;


use yew::html::Href;
use views::loadable::Loadable;

pub struct Model {
    pub page: Page
}

pub enum Page {
    News(NewsModel),
    BucketQuestions(BucketModel),
}

impl<'a> Into<Href> for &'a Page {
    fn into(self) -> Href {
        match *self {
            Page::News(_) => "#/news".into(),
            Page::BucketQuestions(_) => "#/bucket_questions".into(),
        }
    }
}


