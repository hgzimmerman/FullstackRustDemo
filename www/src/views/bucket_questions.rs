use controller::Msg;
use yew::html::Html;

use models::BucketModel;
use views::Viewable;

impl Viewable<Msg> for BucketModel {
    fn view(&self) -> Html<Msg> {

        html!{
            <div class=("container"),>
                { "Buckets??? I hardly know her!" }

                { format!("Your username is {}", self.user_name) }
            </div>
        }
    }
}
