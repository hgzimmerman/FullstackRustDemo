use yew::prelude::*;
use datatypes::bucket::BucketData;
//use util::loadable::Loadable;
use Context;
//use Route;
use util::loadable::Loadable;
use util::uploadable::Uploadable;
use super::BucketModel;
use super::Msg;

use util::button::Button;



pub struct ApprovedBucket(pub BucketData);
pub struct PublicBucket(pub BucketData);

impl Renderable<Context, BucketModel> for Vec<ApprovedBucket>
{
    fn view(&self) -> Html<Context, BucketModel> {
        html! {
            <div class=("flexbox-horiz-wrap"),>
                {for self.iter().map(ApprovedBucket::view)}
            </div>
        }
    }
}
impl Renderable<Context, BucketModel> for Vec<PublicBucket>
{
    fn view(&self) -> Html<Context, BucketModel> {
        html! {
            <div class=("flexbox-horiz-wrap"),>
                {for self.iter().map(PublicBucket::view)}
            </div>
        }
    }
}


impl Renderable<Context, BucketModel> for PublicBucket {
    fn view(&self) -> Html<Context, BucketModel> {
        let bucket_uuid = self.as_ref().uuid.clone();
        html! {
            <div class="public-bucket-card",>
                <div class=("flexbox-vert", "full-height"),>
                    <div class="flexbox-expand",>
                        {&self.as_ref().bucket_name}
                    </div>
                    <div class="flexbox-horiz",>
                        <Button: title="Request Access", onclick=move |_| Msg::RequestToJoinBucket{bucket_uuid} ,/>
                    </div>
                </div>
            </div>
        }
    }
}

impl Renderable<Context, BucketModel> for ApprovedBucket {
    fn view(&self) -> Html<Context, BucketModel> {
        let bucket_uuid = self.as_ref().uuid.clone();
        html! {
            <div
                class="approved-bucket-card",
                onclick = |_| Msg::NavigateToBucket{bucket_uuid}, >

                {&self.as_ref().bucket_name}
            </div>
        }
    }
}


impl AsRef<BucketData> for PublicBucket {
    fn as_ref(&self) -> &BucketData {
        &self.0
    }
}
impl AsRef<BucketData> for ApprovedBucket {
    fn as_ref(&self) -> &BucketData {
        &self.0
    }
}


/// Buckets that are shown for the user to join, or request to be added to.
#[derive(Default)]
pub struct BucketLists {
    /// The approved buckets list contains bucket sessions the user can join.
    pub approved_buckets: Loadable<Vec<ApprovedBucket>>,
    /// The public buckets are buckets that bucket owners have made public.
    /// Users must ask to join these buckets, and they will be approved by the owners of the bucket before the bucket appears in the public bucket list.
    pub public_buckets: Loadable<Vec<PublicBucket>>,
    /// This is just a dumb container for the FT that makes the request to join a bucket.
    pub request_to_join_bucket_action: Uploadable<()>,
}

impl Renderable<Context, BucketModel> for BucketLists {
    fn view(&self) -> Html<Context, BucketModel> {
        html! {
            <div class=("full-height", "full-width", "flexbox-vert", "scrollable"),>
                <div class="flexbox-expand",>
                    <div class=("full-width","light-gray"),>
                        {"Approved Buckets"}
                    </div>
                    {self.approved_buckets.default_view(Vec::<ApprovedBucket>::view)}
                </div>
                <div class="flexbox-expand",>
                    <div class=("full-width","light-gray"),>
                        {"Public Buckets"}
                    </div>
                    {self.public_buckets.default_view(Vec::<PublicBucket>::view)}
                </div>
            </div>
        }
    }
}