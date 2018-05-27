#[macro_use]
extern crate yew;
extern crate failure;
extern crate context;
extern crate wire;
extern crate util;
extern crate routes;

pub use context::datatypes;
pub use context::Context;
pub use routes::bucket::BucketRoute;


use yew::prelude::*;

mod bucket;
mod buckets;
mod new_bucket;
mod bucket_participants;


use util::button::Button;
use bucket::BucketLobby;


use routes::Route;

use context::datatypes::bucket::BucketData;
use util::loadable::Loadable;


use yew::format::Json;
use yew::services::fetch::Response;
use failure::Error;
use context::networking::RequestWrapper;
use wire::bucket::BucketResponse;

//use util::input::InputValidator;
//use util::input::Input;
use util::input::InputState;

use util::uploadable::Uploadable;
use wire::bucket::NewBucketRequest;

use bucket_participants::BucketParticipants;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NewBucket {
    pub name: InputState
}
impl NewBucket {
    pub fn validate_name(name: String) -> Result<String, String> {
        if name.len() < 1 {
            return Err("Bucket Name must have some text.".into())
        }
        Ok(name)
    }
    pub fn validate(&self) -> Result<NewBucketRequest, String> {
        Self::validate_name(self.name.inner_text())?;

        let request = NewBucketRequest {
            bucket_name: self.name.inner_text().clone(),
            is_public: true // By default, all buckets are public, but the option to parameterize it in the UI in the future is possible
        };
        Ok(request)
    }
}

pub struct Buckets {
    joinable_buckets: Loadable<Vec<BucketData>>,
    public_buckets: Loadable<Vec<BucketData>>
}


#[derive(PartialEq, Debug, Clone, Default)]
pub struct Props {
    pub route: BucketRoute
}

pub struct BucketModel {
    bucket_page: BucketPage
}

pub enum BucketPage {
    BucketList(Loadable<Vec<BucketData>>),
    Bucket(Loadable<BucketData>),
    Create(Uploadable<NewBucket>)
}


pub enum Msg {
    NavigateToBucket{bucket_id: i32},
    BucketsReady(Vec<BucketData>),
    BucketsFailed,
    BucketReady(BucketData),
    BucketFailed,
    NavigateToCreateBucket,
    CreateBucket,
    UpdateBucketName(InputState),
}

impl BucketModel {
    fn get_buckets(buckets: &mut Loadable<Vec<BucketData>>, context: &mut Env<Context, Self>) {
        let threads_callback = context.send_back(
            |response: Response<Json<Result<Vec<BucketResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::BucketsReady(
                        data.unwrap()
                            .into_iter()
                            .map(BucketData::from)
                            .collect(),
                    )
                } else {
                    Msg::BucketsFailed
                }
            },
        );

        context.make_logoutable_request(
            buckets,
            RequestWrapper::GetPublicBuckets,
            threads_callback,
        );
    }

    fn get_bucket(bucket: &mut Loadable<BucketData>, bucket_id: i32, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<BucketResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::BucketReady(
                        data.map(BucketData::from).unwrap()
                    )
                } else {
                    Msg::BucketFailed
                }
            },
        );

        context.make_logoutable_request(
            bucket,
            RequestWrapper::GetBucket{bucket_id},
            callback,
        );
    }

    fn create_bucket(new_bucket: &mut Uploadable<NewBucket>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<BucketResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::BucketReady(
                        data.map(BucketData::from).unwrap()
                    )
                } else {
                    Msg::BucketFailed
                }
            },
        );

        let bucket: NewBucket = new_bucket.cloned_inner();

        match bucket.validate() {
            Ok(new_bucket_request) => {
                 context.make_logoutable_request(
                    new_bucket,
                    RequestWrapper::CreateBucket(new_bucket_request),
                    callback,
                );
            }
            Err(error) => {
                new_bucket.set_failed(&error)
            }
        }


    }
}

impl Component<Context> for BucketModel {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        let bucket_page: BucketPage = match props.route {
            BucketRoute::BucketList => {
                let mut buckets = Loadable::default();
                Self::get_buckets(&mut buckets, context);
                BucketPage::BucketList(buckets)
            }
            BucketRoute::Bucket{bucket_id} => {
                let mut bucket = Loadable::default();
                Self::get_bucket(&mut bucket, bucket_id, context);
                BucketPage::Bucket(bucket)
            }
            BucketRoute::Create => {
                BucketPage::Create(Uploadable::default())
            }
        };


        BucketModel {
            bucket_page
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            NavigateToBucket {bucket_id} => context.routing.set_route(Route::Bucket(BucketRoute::Bucket{bucket_id})),
            BucketsReady(buckets) => self.bucket_page = BucketPage::BucketList(Loadable::Loaded(buckets)),
            BucketsFailed => self.bucket_page = BucketPage::BucketList(Loadable::Failed(Some("Failed to load buckets.".to_string()))),
            BucketReady(bucket) => self.bucket_page = BucketPage::Bucket(Loadable::Loaded(bucket)),
            BucketFailed => self.bucket_page = BucketPage::Bucket(Loadable::Failed(Some("Failed to load bucket.".to_string()))),
            NavigateToCreateBucket => context.routing.set_route(Route::Bucket(BucketRoute::Create)),
            CreateBucket => {
                if let BucketPage::Create(ref mut new_bucket) = self.bucket_page {
                    Self::create_bucket(new_bucket, context)
                }
            },
            UpdateBucketName(bucket_name) => {
                if let BucketPage::Create(ref mut new_bucket) = self.bucket_page {
                    new_bucket.as_mut().name = bucket_name;
                } else {
                    context.log("Incoherent state. Expected page to be /create");
                    return false
                }
            }
        }
        true
    }
    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        let bucket_page: BucketPage = match props.route {
            BucketRoute::BucketList => {
                let mut buckets = Loadable::default();
                Self::get_buckets(&mut buckets, context);
                BucketPage::BucketList(buckets)
            }
            BucketRoute::Bucket{bucket_id} => {
                let mut bucket = Loadable::default();
                Self::get_bucket(&mut bucket, bucket_id, context);
                BucketPage::Bucket(bucket)
            }
            BucketRoute::Create => {
                BucketPage::Create(Uploadable::default())
            }
        };
        self.bucket_page = bucket_page;
        true
    }
}
impl Renderable<Context, BucketModel> for BucketModel {
    fn view(&self) -> Html<Context, BucketModel> {

        use self::BucketPage::*;

        fn bucket_lobby_fn(bucket: &BucketData) -> Html<Context, BucketModel> {
            html! {
                <>
                    <BucketLobby: bucket_data=bucket, />
                </>
            }
        }

        let page = match self.bucket_page {
            BucketList(ref buckets) => buckets.default_view(Vec::<BucketData>::view),
            Bucket(ref bucket) => bucket.default_view(bucket_lobby_fn),
            Create(ref new_bucket) => html! {
                <div class="flexbox-center-item",>
                    {new_bucket.default_view(NewBucket::view)}
                </div>
            }
        };

        let title_content = match self.bucket_page {
            BucketList(_) => html! {
                <div class=("flexbox-horiz","full-width"),>
                    <div class="flexbox-expand", >
                        {"Buckets"}
                    </div>
                    <div>
                        <Button: title="Create Bucket", onclick=|_| Msg::NavigateToCreateBucket, />
                    </div>
                </div>
            },
            Bucket(ref bucket) => html! {
                <div class=("flexbox-horiz","full-width"),>
                    <div class="flexbox-expand",>
                    {
                        &if let Loadable::Loaded(bucket_data) = bucket {
                            format!("Bucket: {}", bucket_data.bucket_name)
                        } else {
                            "Bucket: ".into()
                        }
                    }
                    </div>
                    <BucketParticipants: bucket_data=bucket,/>
                </div>
            },
            Create(_) => html! {
                <div>
                    {"Create Bucket"}
                </div>
            }
        };


        html! {
            <div class=("flexbox-vert", "full-height"),>
                <div class="flexbox-horiz",>
                     <div class=("title-bar", "flexbox-center-vert"), > // Title bar
                        {title_content}
                    </div>
                </div>
                <div class=("scrollable", "full-height", "flexbox"),>
                    {page}
                </div>
            </div>
        }

    }
}

