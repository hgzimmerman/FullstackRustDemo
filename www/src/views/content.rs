use controller::Msg;
use yew::html::Html;
use models::Model;
use models::Page;
use bucket_questions;
use views::Viewable;

pub fn view(model: &Model) -> Html<Msg> {

    let page_view = match model.page {
        Page::News(ref news_model) => news_model.view(),
        Page::BucketQuestions(ref bucket_model) => bucket_model.view()
    };

    html! {
        <div>
            {page_view}
        </div>
    }
}