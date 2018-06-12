use Context;
use util::button::Button;
use yew::prelude::*;
use yew::format::Json;

use failure::Error;
use yew::services::fetch::Response;

use util::loadable::Loadable;
use util::uploadable::Uploadable;

use context::datatypes::bucket::BucketData;
use context::networking::RequestWrapper;
use context::datatypes::user::UserData;
use wire::user::UserResponse;
use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;




/// A component for approving and rejecting requests to join buckets.
#[derive(Debug)]
pub struct BucketParticipants {
    users:  Loadable<Vec<UserData>>,
    is_user_bucket_owner: Loadable<bool>,
    bucket_uuid: Option<BucketUuid>,
    remove_user_action: Uploadable<()>
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: Loadable<BucketData>

}

impl Default for BucketParticipants {
    fn default() -> Self {
        BucketParticipants {
            users: Loadable::default(),
            is_user_bucket_owner: Loadable::default(),
            bucket_uuid: None,
            remove_user_action: Uploadable::default()
        }
    }
}

pub enum Msg {
    GetBucketUserData{bucket_uuid: BucketUuid},
    BucketUserDataLoaded(Vec<UserData>),
    BucketUserDataFailed,
    SetIsUserOwner(bool),
    RemoveUserFromBucket{user_uuid: UserUuid}
}

impl BucketParticipants {
    fn get_participants_in_bucket(bucket_uuid: BucketUuid, participants: &mut Loadable<Vec<UserData>>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<Vec<UserResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let new_bucket_users_data: Vec<UserData> = data.map(::wire::convert_vector).unwrap();
                    Msg::BucketUserDataLoaded(new_bucket_users_data)
                } else {
                    Msg::BucketUserDataFailed
                }
            },
        );

        context.make_request_and_set_ft(
            participants,
            RequestWrapper::GetUsersInBucket{bucket_uuid},
            callback,
        );
    }

    fn determine_if_user_is_owner(bucket_uuid: BucketUuid, is_owner: &mut Loadable<bool>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<bool, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::SetIsUserOwner(data.expect("Could not unwrap bool"))
                } else {
                    println!("Could not get user->bucket ownership info");
                    Msg::SetIsUserOwner(false)
                }
            },
        );

        context.make_request_and_set_ft(
            is_owner,
            RequestWrapper::GetIsUserOwnerOfBucket{bucket_uuid},
            callback,
        );
    }

    fn remove_user_from_bucket(bucket_uuid: BucketUuid, user_uuid: UserUuid, remove_user_action: &mut Uploadable<()>, context: &mut Env<Context, Self>) {
        let bucket_uuid: BucketUuid = bucket_uuid;
        let callback = context.send_back(
            move |response: Response<Json<Result<(), Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                Msg::GetBucketUserData {bucket_uuid} // This is lazy, but just get the whole list again.
            },
        );

        context.log(&format!("Removing user from bucket:{}-{}",user_uuid,bucket_uuid));

        context.make_request_and_set_ft(
            remove_user_action,
            RequestWrapper::RemoveUserFromBucket{bucket_uuid, user_uuid},
            callback,
        );
    }
}


impl Component<Context> for BucketParticipants {
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut participants: BucketParticipants = BucketParticipants::default();
        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            Self::get_participants_in_bucket(bucket_data.uuid, &mut participants.users, context); // TODO Possibly don't load this on startup, only do this when opening the pane
            Self::determine_if_user_is_owner(bucket_data.uuid, &mut participants.is_user_bucket_owner, context);
            participants.bucket_uuid = Some(bucket_data.uuid);
        }
        participants
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            GetBucketUserData {bucket_uuid} => {
                Self::get_participants_in_bucket(bucket_uuid, &mut self.users, context);
            }
            BucketUserDataLoaded(bucket_user_data) => {
                self.users = Loadable::Loaded(bucket_user_data);
            },
            BucketUserDataFailed => context.log("Failed to get bucket user data"),
            SetIsUserOwner(is_owner) => self.is_user_bucket_owner = Loadable::Loaded(is_owner),
            RemoveUserFromBucket{user_uuid} => {
                if let Some(bucket_uuid) = self.bucket_uuid {
                    Self::remove_user_from_bucket(bucket_uuid, user_uuid, &mut self.remove_user_action, context)
                } else {
                    context.log("Couldn't remove user because bucket id is unknown.")
                }
            }
        }
        true
    }


    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            Self::get_participants_in_bucket(bucket_data.uuid, &mut self.users, context);
            Self::determine_if_user_is_owner(bucket_data.uuid, &mut self.is_user_bucket_owner, context);
            self.bucket_uuid = Some(bucket_data.uuid);
            true
        } else {
            false
        }
    }
}



impl Renderable<Context, BucketParticipants> for BucketParticipants {
    fn view(&self) -> Html<Context, BucketParticipants> {

        html! {
            <div class="bucket-action-pane",>
                {
                    if let Loadable::Loaded(is_owner) = self.is_user_bucket_owner {
                        if is_owner {
                            self.users.default_view( Self::users_owner_view)
                        }  else {
                            self.users.default_view( Self::users_not_owner_view )
                        }
                    } else {
                        self.users.default_view( Self::users_not_owner_view)
                    }
                }
            </div>
        }
    }
}

impl BucketParticipants {
    fn users_owner_view(users: &Vec<UserData>) -> Html<Context, BucketParticipants> {
        Self::users_view(users, true)
    }
    fn users_not_owner_view(users: &Vec<UserData>) -> Html<Context, BucketParticipants> {
        Self::users_view(users, false)
    }

    fn users_view(users: &Vec<UserData>, is_user_owner: bool) -> Html<Context, BucketParticipants> {

        fn user_view(user: &UserData, is_user_owner: bool) -> Html<Context, BucketParticipants> {
            let user_uuid = user.uuid;
            let delete_button = if is_user_owner {
                html! {
                    <Button: title="Remove", onclick=move |_| Msg::RemoveUserFromBucket{user_uuid} ,/>
                }
            } else {
                ::util::wrappers::empty_vdom_node()
            };

            html!{
                <div class=("flexbox-horiz","full-width"),>
                    <div class="flexbox-expand",>
                        {&user.user_name}
                    </div>
                    <div>
                        {delete_button}
                    </div>
                </div>
            }
        }

        html!{
            <>
                {for users.iter().map(|u| user_view(u, is_user_owner))}
            </>
        }
    }
}

