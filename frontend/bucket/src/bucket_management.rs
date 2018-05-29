use context::datatypes::bucket::BucketUsersData;
use Context;
use util::button::Button;
use yew::prelude::*;
use yew::format::Json;

use failure::Error;
use wire::bucket::BucketUsersResponse;
use yew::services::fetch::Response;

use util::loadable::Loadable;
use util::uploadable::Uploadable;

use context::networking::RequestWrapper;

use context::datatypes::user::UserData;

/// A component for approving and rejecting requests to join buckets.
pub struct BucketManagement {
    bucket_users:  Loadable<Vec<BucketUsersData>>,
    remove_user_action: Uploadable<()>,
    approve_user_action: Uploadable<()>,
    set_public_or_private_action: Uploadable<()>
}

impl Default for BucketManagement {
    fn default() -> Self {
        BucketManagement {
            bucket_users: Loadable::default(),
            remove_user_action: Uploadable::default(),
            approve_user_action: Uploadable::default(),
            set_public_or_private_action: Uploadable::default()
        }
    }
}
pub enum PublicOrPrivate {
    Public,
    Private
}

pub enum Msg {
    GetBucketUsersData,
    BucketUsersDataLoaded(Vec<BucketUsersData>),
    BucketUsersDataFailed,
    GrantUserAccessToBucket{user_id: i32, bucket_id: i32},
    DenyUserAccessToBucket{user_id: i32, bucket_id: i32},
    SetPublicOrPrivate{bucket_id: i32, pub_or_priv: PublicOrPrivate}
}

impl BucketManagement {
    fn get_managable_buckets(loadable_bucket_users: &mut Loadable<Vec<BucketUsersData>>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<Vec<BucketUsersResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let new_bucket_users_data: Vec<BucketUsersData> = data.map(::wire::convert_vector).unwrap();
                    Msg::BucketUsersDataLoaded(new_bucket_users_data)
                } else {
                    Msg::BucketUsersDataFailed
                }
            },
        );

        context.make_logoutable_request(
            loadable_bucket_users,
            RequestWrapper::GetUnapprovedUsersForOwnedBuckets,
            callback,
        );
    }

    fn grant_access_to_user_for_bucket(bucket_id: i32, user_id: i32, approve_user_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            move |response: Response<Json<Result<(), Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::GetBucketUsersData // This is lazy, but just get the whole list again.
            },
        );

        context.make_logoutable_request(
            approve_user_action,
            RequestWrapper::ApproveUserForBucket{bucket_id, user_id},
            callback,
        );
    }

    fn remove_user_from_bucket(bucket_id: i32, user_id: i32, remove_user_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
        let bucket_id: i32 = bucket_id;
        let callback = context.send_back(
            move |response: Response<Json<Result<(), Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::GetBucketUsersData // This is lazy, but just get the whole list again.
            },
        );

        context.make_logoutable_request(
            remove_user_action,
            RequestWrapper::RemoveUserFromBucket{bucket_id, user_id},
            callback,
        );
    }

    fn set_public_or_private(bucket_id: i32, pub_or_priv: PublicOrPrivate, set_public_or_private_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
        let bucket_id: i32 = bucket_id;
        let callback = context.send_back(
            move |response: Response<Json<Result<(), Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::GetBucketUsersData // This is lazy, but just get the whole list again.
            },
        );

        let is_public = match pub_or_priv {
            PublicOrPrivate::Public => true,
            PublicOrPrivate::Private => false
        };

        context.make_logoutable_request(
            set_public_or_private_action,
            RequestWrapper::SetBucketPublicStatus{bucket_id, is_public},
            callback,
        );
    }
}


impl Component<Context> for BucketManagement {
    type Msg = Msg;
    type Properties = ();


    fn create(_props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut management: BucketManagement = BucketManagement::default();

        Self::get_managable_buckets(&mut management.bucket_users, context);
        management
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            GetBucketUsersData => {
                Self::get_managable_buckets(&mut self.bucket_users, context);
            }
            BucketUsersDataLoaded(bucket_user_data) => {
                self.bucket_users = Loadable::Loaded(bucket_user_data)
            },
            BucketUsersDataFailed => context.log("Failed to get bucket user data"),
            GrantUserAccessToBucket{bucket_id, user_id} => {
                Self::grant_access_to_user_for_bucket(bucket_id, user_id, &mut self.approve_user_action, context)
            }
            DenyUserAccessToBucket{bucket_id, user_id} => {
                Self::remove_user_from_bucket(bucket_id, user_id, &mut self.remove_user_action, context)
            },
            SetPublicOrPrivate {bucket_id, pub_or_priv} => {
                Self::set_public_or_private(bucket_id, pub_or_priv, &mut self.set_public_or_private_action, context)
            }
        }
        true
    }


    fn change(&mut self, _props: Self::Properties, _context: &mut Env<Context, Self>) -> ShouldRender {
        false
    }
}



impl Renderable<Context, BucketManagement> for BucketManagement {
    fn view(&self) -> Html<Context, BucketManagement> {

        html! {
            <div class="bucket-action-pane",>
                { self.bucket_users.default_view( Self::buckets_view ) }
            </div>
        }
    }
}

impl BucketManagement {

    fn buckets_view(buckets: &Vec<BucketUsersData>) -> Html<Context, BucketManagement> {

        fn bucket_view(bucket_user_data: &BucketUsersData) -> Html<Context, BucketManagement> {
            let bucket_id = bucket_user_data.bucket.id;
            let button = if bucket_user_data.bucket.is_public {
                html! {
                    <Button: title="Lock", onclick= move |_| Msg::SetPublicOrPrivate{bucket_id, pub_or_priv: PublicOrPrivate::Private}, />
                }
            } else {
                html! {
                    <Button: title="Unlock", onclick= move |_| Msg::SetPublicOrPrivate{bucket_id, pub_or_priv: PublicOrPrivate::Public}, />
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
                    {BucketManagement::users_view(&bucket_user_data.users, bucket_user_data.bucket.id)}
                </div>
            }
        }
        html! {
            <>
                {for buckets.iter().map(bucket_view)}
            </>
        }
    }

    fn users_view(users: &Vec<UserData>, bucket_id: i32) -> Html<Context, BucketManagement> {

        fn user_view(user: &UserData, bucket_id: i32) -> Html<Context, BucketManagement> {
            let user_id = user.id;

            html!{
                <div class=("flexbox-horiz","full-width"),>
                    <div class="flexbox-expand",>
                        {&user.user_name}
                    </div>
                    <div>
                        <Button: title="Approve", onclick=move |_| Msg::GrantUserAccessToBucket{bucket_id, user_id} ,/>
                        <Button: title="Deny", onclick=move |_| Msg::DenyUserAccessToBucket{bucket_id, user_id} ,/>
                    </div>
                </div>
            }
        }

        html!{
            <>
                {for users.iter().map(|u| user_view(u, bucket_id))}
            </>
        }
    }
}

