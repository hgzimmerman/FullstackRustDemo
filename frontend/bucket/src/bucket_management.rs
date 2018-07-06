use common::datatypes::bucket::BucketUsersData;
//use Context;
use util::button::Button;
use yew::prelude::*;
use yew::format::Json;

use failure::Error;
use wire::bucket::BucketUsersResponse;
use yew::services::fetch::Response;

use util::loadable::Loadable;
use util::uploadable::Uploadable;

//use context::networking::RequestWrapper;

use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;
use common::datatypes::user::UserData;

use requests::BucketRequest;
use common::fetch::Networking;
use common::fetch::FetchResponse;

/// A component for approving and rejecting requests to join buckets.
pub struct BucketManagement {
    bucket_users:  Loadable<Vec<BucketUsersData>>,
    remove_user_action: Uploadable<()>,
    approve_user_action: Uploadable<()>,
    set_public_or_private_action: Uploadable<()>,
    networking: Networking,
    link: ComponentLink<BucketManagement>,
}

//impl Default for BucketManagement {
//    fn default() -> Self {
//        BucketManagement {
//            bucket_users: Loadable::default(),
//            remove_user_action: Uploadable::default(),
//            approve_user_action: Uploadable::default(),
//            set_public_or_private_action: Uploadable::default()
//        }
//    }
//}
pub enum PublicOrPrivate {
    Public,
    Private
}

pub enum Msg {
    GetBucketUsersData,
//    BucketUsersDataLoaded(Vec<BucketUsersData>),
//    BucketUsersDataFailed,
    HandleGetBucketUsersDataResponse(FetchResponse<Vec<BucketUsersResponse>>),
    GrantUserAccessToBucket{user_uuid: UserUuid, bucket_uuid: BucketUuid},
    HandleGrantUserAccessResponse(FetchResponse<()>),
    DenyUserAccessToBucket{user_uuid: UserUuid, bucket_uuid: BucketUuid},
    HandleDenyUserAccessResponse(FetchResponse<()>),
    SetPublicOrPrivate{bucket_uuid: BucketUuid, pub_or_priv: PublicOrPrivate},
    HandleSetPublicityResponse(FetchResponse<()>),
    NoOp
}
impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl BucketManagement {
//    fn get_managable_buckets(loadable_bucket_users: &mut Loadable<Vec<BucketUsersData>>, context: &mut Env<Context, Self>) {
//        let callback = context.send_back(
//            |response: Response<Json<Result<Vec<BucketUsersResponse>, Error>>>| {
//                let (meta, Json(data)) = response.into_parts();
//                println!("META: {:?}, {:?}", meta, data);
//                if meta.status.is_success() {
//                    let new_bucket_users_data: Vec<BucketUsersData> = data.map(::wire::convert_vector).unwrap();
//                    Msg::BucketUsersDataLoaded(new_bucket_users_data)
//                } else {
//                    Msg::BucketUsersDataFailed
//                }
//            },
//        );
//
//        context.make_request_and_set_ft(
//            loadable_bucket_users,
//            BucketRequest::GetUnapprovedUsersForOwnedBuckets,
//            callback,
//        );
//    }
//
//    fn grant_access_to_user_for_bucket(bucket_uuid: BucketUuid, user_uuid: UserUuid, approve_user_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
//        let callback = context.send_back(
//            move |response: Response<Json<Result<(), Error>>>| {
//                let (meta, Json(data)) = response.into_parts();
//                println!("META: {:?}, {:?}", meta, data);
//                Msg::GetBucketUsersData // This is lazy, but just get the whole list again.
//            },
//        );
//
//        context.make_request_and_set_ft(
//            approve_user_action,
//            BucketRequest::ApproveUserForBucket{bucket_uuid, user_uuid},
//            callback,
//        );
//    }
//
//    fn remove_user_from_bucket(bucket_uuid: BucketUuid, user_uuid: UserUuid, remove_user_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
//        let bucket_uuid: BucketUuid = bucket_uuid;
//        let callback = context.send_back(
//            move |response: Response<Json<Result<(), Error>>>| {
//                let (meta, Json(data)) = response.into_parts();
//                println!("META: {:?}, {:?}", meta, data);
//                Msg::GetBucketUsersData // This is lazy, but just get the whole list again.
//            },
//        );
//
//        context.make_request_and_set_ft(
//            remove_user_action,
//            BucketRequest::RemoveUserFromBucket{bucket_uuid, user_uuid},
//            callback,
//        );
//    }
//
//    fn set_public_or_private(bucket_uuid: BucketUuid, pub_or_priv: PublicOrPrivate, set_public_or_private_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
//        let bucket_uuid: BucketUuid = bucket_uuid;
//        let callback = context.send_back(
//            move |response: Response<Json<Result<(), Error>>>| {
//                let (meta, Json(data)) = response.into_parts();
//                println!("META: {:?}, {:?}", meta, data);
//                Msg::GetBucketUsersData // This is lazy, but just get the whole list again.
//            },
//        );
//
//        let is_public = match pub_or_priv {
//            PublicOrPrivate::Public => true,
//            PublicOrPrivate::Private => false
//        };
//
//        context.make_request_and_set_ft(
//            set_public_or_private_action,
//            BucketRequest::SetBucketPublicStatus{bucket_uuid, is_public},
//            callback,
//        );
//    }
}


impl Component for BucketManagement {
    type Message = Msg;
    type Properties = ();


    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut management = BucketManagement {
            bucket_users: Loadable::default(),
            remove_user_action: Uploadable::default(),
            approve_user_action: Uploadable::default(),
            set_public_or_private_action: Uploadable::default(),
            networking: Networking::new(&link),
            link: link,
        };

//        Self::get_managable_buckets(&mut management.bucket_users, context);
        // Get the buckets
        management.networking.fetch(
            BucketRequest::GetUnapprovedUsersForOwnedBuckets,
            |r| Msg::HandleGetBucketUsersDataResponse(r),
            &management.link
        );
        management
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        use self::Msg::*;
        match msg {
            GetBucketUsersData => {
//                Self::get_managable_buckets(&mut self.bucket_users, context);
                self.networking.fetch(
                    BucketRequest::GetUnapprovedUsersForOwnedBuckets,
                    |r| Msg::HandleGetBucketUsersDataResponse(r),
                    &self.link
                );
            }
//            BucketUsersDataLoaded(bucket_user_data) => {
//                self.bucket_users = Loadable::Loaded(bucket_user_data)
//            },
//            BucketUsersDataFailed => warn!("Failed to get bucket user data"),
            HandleGetBucketUsersDataResponse(response) => {
                let response: FetchResponse<Vec<BucketUsersData>> = response.map(::wire::convert_vector);
                self.bucket_users = Loadable::from_fetch_response(response)
            }
            GrantUserAccessToBucket{bucket_uuid, user_uuid} => {
//                Self::grant_access_to_user_for_bucket(bucket_uuid, user_uuid, &mut self.approve_user_action, context)
                self.networking.fetch(
                    BucketRequest::ApproveUserForBucket{bucket_uuid, user_uuid},
                    |r| Msg::HandleGrantUserAccessResponse(r),
                    &self.link
                );
            }
            HandleGrantUserAccessResponse(response) => {
                self.approve_user_action.handle_fetch_response(response.clone());
                if let FetchResponse::Success(_) = response {
                   self.update(GetBucketUsersData);
                }
            }
            DenyUserAccessToBucket{bucket_uuid, user_uuid} => {
//                Self::remove_user_from_bucket(bucket_uuid, user_uuid, &mut self.remove_user_action, context)
                self.networking.fetch(
                    BucketRequest::RemoveUserFromBucket{bucket_uuid, user_uuid},
                    |r| Msg::HandleSetPublicityResponse(r),
                    &self.link
                );
            },
            HandleDenyUserAccessResponse(response) => {
                self.remove_user_action.handle_fetch_response(response.clone());
                if let FetchResponse::Success(_) = response {
                   self.update(GetBucketUsersData);
                }
            }
            SetPublicOrPrivate {bucket_uuid, pub_or_priv} => {
                let is_public: bool = match pub_or_priv {
                    PublicOrPrivate::Public => true,
                    PublicOrPrivate::Private => false
                };
                self.networking.fetch(
                    BucketRequest::SetBucketPublicStatus{bucket_uuid, is_public},
                    |r| Msg::HandleSetPublicityResponse(r),
                    &self.link
                );
//                Self::set_public_or_private(bucket_uuid, pub_or_priv, &mut self.set_public_or_private_action, context)
            }
            HandleSetPublicityResponse(response) => {
                self.set_public_or_private_action.handle_fetch_response(response.clone());
                if let FetchResponse::Success(_) = response {
                   self.update(GetBucketUsersData);
                }
            }
            NoOp => return false
        }
        true
    }


    fn change(&mut self, _props: Self::Properties, ) -> ShouldRender {
        false
    }
}



impl Renderable<BucketManagement> for BucketManagement {
    fn view(&self) -> Html<BucketManagement> {

        html! {
            <div class="bucket-action-pane",>
                { self.bucket_users.default_view( Self::buckets_view ) }
            </div>
        }
    }
}

impl BucketManagement {

    fn buckets_view(buckets: &Vec<BucketUsersData>) -> Html<BucketManagement> {

        fn bucket_view(bucket_user_data: &BucketUsersData) -> Html<BucketManagement> {
            let bucket_uuid = bucket_user_data.bucket.uuid;
            let button = if bucket_user_data.bucket.is_public {
                html! {
                    <Button: title="Lock", onclick= move |_| Msg::SetPublicOrPrivate{bucket_uuid, pub_or_priv: PublicOrPrivate::Private}, />
                }
            } else {
                html! {
                    <Button: title="Unlock", onclick= move |_| Msg::SetPublicOrPrivate{bucket_uuid, pub_or_priv: PublicOrPrivate::Public}, />
                }
            };

            html! {
                <div class=("flexbox-vert", "full-width"),>
                    <div class=("flexbox-horiz", "full-width"), >
                        <div class=("flexbox-expand"),>
                            {&bucket_user_data.bucket.bucket_name}
                        </div>
                        {button}
                    </div>
                    {BucketManagement::users_view(&bucket_user_data.users, bucket_user_data.bucket.uuid)}
                </div>
            }
        }
        html! {
            <>
                {for buckets.iter().map(bucket_view)}
            </>
        }
    }

    fn users_view(users: &Vec<UserData>, bucket_uuid: BucketUuid) -> Html<BucketManagement> {

        fn user_view(user: &UserData, bucket_uuid: BucketUuid) -> Html<BucketManagement> {
            let user_uuid = user.uuid;

            html!{
                <div class=("flexbox-horiz","full-width"),>
                    <div class="flexbox-expand",>
                        {&user.user_name}
                    </div>
                    <div>
                        <Button: title="Approve", onclick=move |_| Msg::GrantUserAccessToBucket{bucket_uuid, user_uuid} ,/>
                        <Button: title="Deny", onclick=move |_| Msg::DenyUserAccessToBucket{bucket_uuid, user_uuid} ,/>
                    </div>
                </div>
            }
        }

        html!{
            <>
                {for users.iter().map(|u| user_view(u, bucket_uuid))}
            </>
        }
    }
}

