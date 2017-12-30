use controller::{Msg, NewsMsg};
use yew::html::Html;
use models::{NewsModel, Article};
use views::Viewable;


impl Viewable<Msg> for NewsModel {
    fn view(&self) -> Html<Msg> {
        html!{
            <div class=("container"),>
                {self.article.view()}
                <button onclick=|_| Msg::News(NewsMsg::FetchArticle{id: "Test".to_string()} ),>{ "Fetch Data" }</button>
            </div>
        }
    }
}

