use common::datatypes::bucket::BucketUsersData;
//use Context;
use util::button::Button;
use yew::prelude::*;

use wire::bucket::BucketUsersResponse;

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


pub enum Publicity {
    Public,
    Private
}

pub enum Msg {
    GetBucketUsersData,
    HandleGetBucketUsersDataResponse(FetchResponse<Vec<BucketUsersData>>),
    GrantUserAccessToBucket{user_uuid: UserUuid, bucket_uuid: BucketUuid},
    HandleGrantUserAccessResponse(FetchResponse<()>),
    DenyUserAccessToBucket{user_uuid: UserUuid, bucket_uuid: BucketUuid},
    HandleDenyUserAccessResponse(FetchResponse<()>),
    SetPublicOrPrivate{bucket_uuid: BucketUuid, publicity: Publicity },
    HandleSetPublicityResponse(FetchResponse<()>),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl BucketManagement {
    fn get_manageable_buckets(networking: &mut Networking, link: &ComponentLink<Self>) {
        networking.fetch(
            BucketRequest::GetUnapprovedUsersForOwnedBuckets,
            |r: FetchResponse<Vec<BucketUsersResponse>>| {
                let r: FetchResponse<Vec<BucketUsersData>> = r.map(::wire::convert_vector);
                Msg::HandleGetBucketUsersDataResponse(r)
            },
            link
        );
    }

    fn grant_access_to_user_for_bucket(&mut self, bucket_uuid: BucketUuid, user_uuid: UserUuid) {
        self.networking.fetch(
            BucketRequest::ApproveUserForBucket{bucket_uuid, user_uuid},
            |r| Msg::HandleGrantUserAccessResponse(r),
            &self.link
        );
    }
    fn remove_user_from_bucket(&mut self, bucket_uuid: BucketUuid, user_uuid: UserUuid) {
        self.networking.fetch(
            BucketRequest::RemoveUserFromBucket{bucket_uuid, user_uuid},
            |r| Msg::HandleDenyUserAccessResponse(r),
            &self.link
        );
    }

    fn set_public_or_private(&mut self, bucket_uuid: BucketUuid, publicity: Publicity) {
        let is_public: bool = match publicity {
            Publicity::Public => true,
            Publicity::Private => false
        };
        self.networking.fetch(
            BucketRequest::SetBucketPublicStatus{bucket_uuid, is_public},
            |r| Msg::HandleSetPublicityResponse(r),
            &self.link
        );
    }
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
            link,
        };

        // Get the buckets
        Self::get_manageable_buckets(&mut management.networking, &management.link);
        management
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        use self::Msg::*;
        match msg {
            GetBucketUsersData => {
                Self::get_manageable_buckets(&mut self.networking, &self.link);
            }
            HandleGetBucketUsersDataResponse(response) => {
                self.bucket_users = Loadable::from_fetch_response(response)
            }
            GrantUserAccessToBucket{bucket_uuid, user_uuid} => {
                self.grant_access_to_user_for_bucket(bucket_uuid, user_uuid);
            }
            HandleGrantUserAccessResponse(response) => {
                self.approve_user_action.handle_fetch_response(response.clone());
                if let FetchResponse::Success(_) = response {
                   self.update(GetBucketUsersData);
                }
            }
            DenyUserAccessToBucket{bucket_uuid, user_uuid} => {
                self.remove_user_from_bucket(bucket_uuid, user_uuid);
            },
            HandleDenyUserAccessResponse(response) => {
                self.remove_user_action.handle_fetch_response(response.clone());
                if let FetchResponse::Success(_) = response {
                   self.update(GetBucketUsersData);
                }
            }
            SetPublicOrPrivate {bucket_uuid, publicity} => {
                self.set_public_or_private(bucket_uuid, publicity)
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
                    <Button: title="Lock", onclick= move |_| Msg::SetPublicOrPrivate{bucket_uuid, publicity: Publicity::Private}, />
                }
            } else {
                html! {
                    <Button: title="Unlock", onclick= move |_| Msg::SetPublicOrPrivate{bucket_uuid, publicity: Publicity::Public}, />
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

