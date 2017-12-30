use msg::Msg;
use yew::html::Html;
use models::{NewsModel,};
use views::Viewable;


impl Viewable<Msg> for NewsModel {
    fn view(&self) -> Html<Msg> {
        html!{
            <div class=("container"),>
                {self.article.view()}
            </div>
        }
    }
}

