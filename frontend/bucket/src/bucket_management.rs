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

use context::datatypes::bucket::BucketData;
use context::networking::RequestWrapper;
use context::datatypes::user::UserData;
use wire::user::UserResponse;


/// A component for approving and rejecting requests to join buckets.
pub struct BucketManagement {
    bucket_users:  Loadable<Vec<BucketUsersData>>,
    is_open: bool,
    remove_user_action: Uploadable<()>,
    approve_user_action: Uploadable<()>
}

impl Default for BucketManagement {
    fn default() -> Self {
        BucketManagement {
            bucket_users: Loadable::default(),
            is_open: false,
            remove_user_action: Uploadable::default(),
            approve_user_action: Uploadable::default()
        }
    }
}

pub enum Msg {
    GetBucketUsersData,
    BucketUsersDataLoaded(Vec<BucketUsersData>),
    BucketUsersDataFailed,
    GrantUserAccessToBucket{user_id: i32, bucket_id: i32},
    DenyUserAccessToBucket{user_id: i32, bucket_id: i32}
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
}


impl Component<Context> for BucketManagement {
    type Msg = Msg;
    type Properties = ();


    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
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
            <div style="position: absolute; top: 40px; width: 300px; left: -160px; border: 1px solid black; min-height: 200px; background-color: white",>
                { self.bucket_users.default_view( Self::buckets_view ) }
            </div>
        }
    }
}

impl BucketManagement {

    fn buckets_view(buckets: &Vec<BucketUsersData>) -> Html<Context, BucketManagement> {

        fn bucket_view(bucket_user_data: &BucketUsersData) -> Html<Context, BucketManagement> {
            html! {
                <div class=("flexbox-horiz", "full-width"),>
                    {&bucket_user_data.bucket.bucket_name}
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
                        <Button: title="Remove", onclick=move |_| Msg::DenyUserAccessToBucket{bucket_id, user_id} ,/>
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

