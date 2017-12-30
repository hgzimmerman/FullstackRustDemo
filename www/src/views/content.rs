use msg::Msg;
use yew::html::Html;
use models::Model;
use models::Page;
use bucket_questions;
use views::Viewable;

pub fn view(model: &Model) -> Html<Msg> {

    let inner_view = match model.page {
        Page::News(ref news_model) => news_model.view(),
        Page::BucketQuestions => bucket_questions::view()
    };

    html! {
        <div>
            {inner_view}
        </div>
    }
}