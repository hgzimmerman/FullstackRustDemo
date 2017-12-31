use controller::Msg;
use yew::html::Html;
use views::Viewable;

use models::Article;


impl Viewable<Msg> for Article {
    fn view(&self) -> Html<Msg> {

        html!{
            <div>
                <h2>
                    { self.title.clone() }
                </h2>

                <h6>
                    { format!("By: {}", self.author)}
                </h6>
                <h6>
                    { format!("Published: {}", self.publish_date)}
                </h6>

                <div>
                    { self.content.clone() }
                </div>

            </div>
        }
    }
}