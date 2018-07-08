#[macro_use]
extern crate yew;
extern crate yew_router;
extern crate failure;
//extern crate context;
extern crate wire;
extern crate identifiers;
extern crate util;
//extern crate routes;
extern crate common;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate log;

pub use common::datatypes;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router_agent::RouterSenderBase;

mod bucket;
mod buckets;
mod new_bucket;
mod bucket_participants;
mod bucket_management;
mod requests;


use util::button::Button;
use bucket::BucketLobby;
use util::loadable::Loadable;
use wire::bucket::BucketResponse;
//use util::input::InputValidator;
//use util::input::Input;
use util::input::InputState;
use util::uploadable::Uploadable;
use wire::bucket::NewBucketRequest;
use bucket_participants::BucketParticipants;
use bucket_management::BucketManagement;
use buckets::BucketLists;
use buckets::ApprovedBucket;
use buckets::PublicBucket;
use identifiers::bucket::BucketUuid;

use common::fetch::Networking;
use common::fetch::FetchResponse;
use common::datatypes::bucket::BucketData;

use requests::BucketRequest;
use yew_router::components::RouterButton;




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

#[derive(PartialEq, Clone, Debug)]
pub enum DropDownPaneVariant {
    ManageBuckets,
    ViewParticipants,
    Closed
}


#[derive(Debug, PartialEq, Clone)]
pub enum BucketRoute {
    BucketList,
    Bucket{bucket_uuid: BucketUuid},
    Create
}
impl Default for BucketRoute {
    fn default() -> Self {
        BucketRoute::BucketList
    }
}

impl Routable for BucketModel {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if let Some(seg_2) = route.path_segments.get(1) {
            if let Ok(bucket_uuid) = BucketUuid::parse_str(&seg_2) {
                Some(BucketRoute::Bucket{bucket_uuid})
            } else if seg_2 == "create" {
                Some(BucketRoute::Create)
            } else {
                None
            }
        } else {
            Some(BucketRoute::BucketList)
        }
    }
    fn will_try_to_route(route: &Route) -> bool {
        if let Some(seg_1) = route.path_segments.get(0) {
            seg_1.as_str() == "bucket"
        } else {
            false
        }
    }
}

pub struct BucketModel {
    bucket_page: BucketPage,
    drop_down_state: DropDownPaneVariant,
    networking: Networking,
    link: ComponentLink<BucketModel>,
    router: RouterSenderBase<()>
}



pub enum BucketPage {
    BucketList(BucketLists),
    Bucket(Loadable<BucketData>),
    Create(Uploadable<NewBucket>)
}


pub enum Msg {
    NavigateToBucket{bucket_uuid: BucketUuid},
    NavigateToCreateBucket,
    HandleGetPublicBucketsResponse(FetchResponse<Vec<BucketResponse>>),
    HandleGetApprovedBucketsResponse(FetchResponse<Vec<BucketResponse>>),
    HandleGetBucketResponse(FetchResponse<BucketData>),
    HandleJoinBucketResponse(FetchResponse<()>),
    CreateBucket,
    UpdateBucketName(InputState),
    ChangeDropDownState(DropDownPaneVariant),
    RequestToJoinBucket{bucket_uuid: BucketUuid},
    NoOp // TODO remove me
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl BucketModel {
    /// Gets the list of buckets the user can request to join.
    fn get_public_buckets(networking: &mut Networking, link: &ComponentLink<Self>)  {
        networking.fetch(BucketRequest::GetPublicBuckets, |r| Msg::HandleGetPublicBucketsResponse(r) , link);
    }

    /// Gets the list of buckets the user can join.
    fn get_approved_buckets(networking: &mut Networking, link: &ComponentLink<Self>) {
        networking.fetch(BucketRequest::GetBucketsForUser, |r| Msg::HandleGetApprovedBucketsResponse(r) , link);
    }

    fn get_bucket(bucket_uuid: BucketUuid, networking: &mut Networking, link: &ComponentLink<Self>) {
        networking.fetch(
            BucketRequest::GetBucket{bucket_uuid},
            |r: FetchResponse<BucketResponse>| Msg::HandleGetBucketResponse(r.map(BucketData::from)),
            link
        );
    }

    fn create_bucket(&mut self, bucket: NewBucket) {
        match bucket.validate() {
            Ok(new_bucket_request) => {
                self.networking.fetch(BucketRequest::CreateBucket(new_bucket_request), |r: FetchResponse<BucketResponse>| Msg::HandleGetBucketResponse(r.map(BucketData::from)) , &self.link);
            }
            Err(error) => {
                if let BucketPage::Create(ref mut new_bucket) = self.bucket_page {
                    new_bucket.set_failed(&error)
                }
            }
        }
    }
    fn request_to_join_bucket(&mut self, bucket_uuid: BucketUuid) {
        self.networking.fetch(BucketRequest::CreateJoinBucketRequest{bucket_uuid}, |r: FetchResponse<()>| Msg::HandleJoinBucketResponse(r) , &self.link);
    }
}

impl Component for BucketModel {
    type Message = Msg;
    type Properties = BucketRoute;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut networking = Networking::new(&link);

        let bucket_page: BucketPage = match props {
            BucketRoute::BucketList => {
                Self::get_public_buckets(&mut networking, &link);
                Self::get_approved_buckets(&mut networking, &link);
                BucketPage::BucketList(BucketLists::default())
            }
            BucketRoute::Bucket{bucket_uuid} => {
                Self::get_bucket(bucket_uuid, &mut networking, &link);
                BucketPage::Bucket(Loadable::default())
            }
            BucketRoute::Create => {
                BucketPage::Create(Uploadable::default())
            }
        };

        let router_cb = link.send_back(|_| Msg::NoOp);
        BucketModel {
            bucket_page,
            drop_down_state: DropDownPaneVariant::Closed,
            networking,
            router: RouterSenderBase::<()>::new(router_cb),
            link
        }


    }

    fn update(&mut self, msg: Msg ) -> ShouldRender {
        use self::Msg::*;
        match msg {
            NavigateToBucket {bucket_uuid} => self.router.send(RouterRequest::ChangeRoute(Route::parse(&format!("bucket/{}", bucket_uuid)))),//context.routing.set_route(Route::Bucket(BucketRoute::Bucket{bucket_uuid}).to_route().to_string()),
            NavigateToCreateBucket => self.router.send(RouterRequest::ChangeRoute(Route::parse("bucket/create"))),
            HandleGetPublicBucketsResponse(buckets_response) => {

                let public_buckets_response: FetchResponse<Vec<PublicBucket>> = buckets_response
                    .map(|x: Vec<BucketResponse>| x
                        .into_iter()
                        .map(BucketData::from)
                        .map(PublicBucket).collect()
                    );
                if let BucketPage::BucketList(ref mut bucket_list) = self.bucket_page {
                    bucket_list.public_buckets = Loadable::from_fetch_response(public_buckets_response);
                } else {
                    let mut bucket_lists = BucketLists::default();
                    bucket_lists.public_buckets = Loadable::from_fetch_response(public_buckets_response);
                    self.bucket_page = BucketPage::BucketList(bucket_lists)
                }
            }
            HandleGetApprovedBucketsResponse(buckets_response) => {
                let approved_buckets_response: FetchResponse<Vec<ApprovedBucket>> = buckets_response
                    .map(|x: Vec<BucketResponse>| x
                        .into_iter()
                        .map(BucketData::from)
                        .map(ApprovedBucket).collect()
                    );

                if let BucketPage::BucketList(ref mut bucket_list) = self.bucket_page {
                    bucket_list.approved_buckets = Loadable::from_fetch_response(approved_buckets_response);
                } else {
                    let mut bucket_lists = BucketLists::default();
                    bucket_lists.approved_buckets = Loadable::from_fetch_response(approved_buckets_response);
                    self.bucket_page = BucketPage::BucketList(bucket_lists)
                }
            }
            HandleGetBucketResponse(bucket_data_response) => {
                self.bucket_page = BucketPage::Bucket(Loadable::from_fetch_response(bucket_data_response))
            }
            CreateBucket => {
                let new_bucket_option: Option<NewBucket> = if let BucketPage::Create(ref mut new_bucket) = self.bucket_page {
                    Some(new_bucket.cloned_inner())
                } else {
                    None
                };


                if let Some(new_bucket) = new_bucket_option {
                    self.create_bucket(new_bucket)
                } else {
                    warn!("app in indeterminate state");
                }
            },
            UpdateBucketName(bucket_name) => {
                if let BucketPage::Create(ref mut new_bucket) = self.bucket_page {
                    new_bucket.as_mut().name = bucket_name;
                } else {
                    warn!("Incoherent state. Expected page to be /create");
                    return false
                }
            }
            ChangeDropDownState(drop_down_state) => {
                if self.drop_down_state == drop_down_state {
                    self.drop_down_state = DropDownPaneVariant::Closed // close the drop down pane if the current one is already selected
                } else {
                    self.drop_down_state = drop_down_state
                }
            }
            RequestToJoinBucket {bucket_uuid} => {
                self.request_to_join_bucket(bucket_uuid)
//                self.networking.fetch(BucketRequest::CreateJoinBucketRequest{bucket_uuid}, |r: FetchResponse<()>| Msg::HandleJoinBucketResponse(r) , &self.link);
            }
            HandleJoinBucketResponse(_response) => {
//                if let BucketPage::BucketList(ref mut bucket_lists) = self.bucket_page {
//                    bucket_lists
//                }
                //TODO this used to be a "noop" but it should probably do something
            }
            NoOp => {}
        }
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let bucket_page: BucketPage = match props {
            BucketRoute::BucketList => {
                Self::get_public_buckets(&mut self.networking, &self.link);
                Self::get_approved_buckets(&mut self.networking, &self.link);
                BucketPage::BucketList(BucketLists::default())
            }
            BucketRoute::Bucket{bucket_uuid} => {
                Self::get_bucket(bucket_uuid, &mut self.networking, &self.link);
                BucketPage::Bucket(Loadable::default())
            }
            BucketRoute::Create => {
                BucketPage::Create(Uploadable::default())
            }
        };
        self.bucket_page = bucket_page;
        true
    }
}
impl Renderable<BucketModel> for BucketModel {
    fn view(&self) -> Html<BucketModel> {

        use self::BucketPage::*;

        fn bucket_lobby_fn(bucket: &BucketData) -> Html<BucketModel> {
            html! {
                <>
                    <BucketLobby: bucket_data=bucket, />
                </>
            }
        }

        let page = match self.bucket_page {
            BucketList(ref buckets) => buckets.view(),
            Bucket(ref bucket) => bucket.default_view(bucket_lobby_fn),
            Create(ref new_bucket) => html! {
                <div class="flexbox-center-item",>
                    {new_bucket.default_view(NewBucket::view)}
                </div>
            }
        };


        let pane = match self.drop_down_state {
            DropDownPaneVariant::Closed => ::util::wrappers::empty_vdom_node(),
            DropDownPaneVariant::ManageBuckets => html! {
                <BucketManagement: />
            },
            DropDownPaneVariant::ViewParticipants => {
                if let Bucket(ref bucket) = self.bucket_page {
                    html! {
                        <BucketParticipants: bucket_data=bucket,/>
                    }
                } else {
                    ::util::wrappers::empty_vdom_node()
                }
            }
        };


        let title_content = match self.bucket_page {
            BucketList(_) => html! {
                <div class=("flexbox-horiz","full-width"),>
                    <div class="flexbox-expand", >
                        {"Buckets"}
                    </div>
                    <div>
                        <RouterButton: text="Create Bucket", route=Route::parse("bucket/create"), />
                    </div>
                    <div style="position: relative",>
                        <Button: title="Manage", onclick=|_| Msg::ChangeDropDownState(DropDownPaneVariant::ManageBuckets), />
                        {pane}
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
                    <div style="position: relative",>

                        <Button: title="Manage", onclick=|_| Msg::ChangeDropDownState(DropDownPaneVariant::ManageBuckets), />
                        <Button: title="Participants", onclick=|_| Msg::ChangeDropDownState(DropDownPaneVariant::ViewParticipants), />
                        {pane}
                    </div>
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

