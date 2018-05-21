use yew::prelude::*;
use datatypes::bucket::BucketData;
use util::loadable::Loadable;
use Context;
//use Route;

use super::BucketModel;
use super::Msg;

impl Renderable<Context, BucketModel> for Vec<BucketData> {
    fn view(&self) -> Html<Context, BucketModel> {
        fn buckets(buckets: &Vec<BucketData>) -> Html<Context, BucketModel> {
             html! {
                {for buckets.iter().map(BucketData::view)}
             }
        }

        html! {
            <div class=("flexbox-horiz-wrap", "full-height"),>
                {buckets(self)}
            </div>
        }
    }
}


impl Renderable<Context, BucketModel> for BucketData {
    fn view(&self) -> Html<Context, BucketModel> {
        let bucket_id = self.id.clone();
        html! {
            <div class="bucket-card", onclick=move |_| Msg::NavigateToBucket{bucket_id}, >
                {&self.bucket_name}
            </div>
        }
    }
}