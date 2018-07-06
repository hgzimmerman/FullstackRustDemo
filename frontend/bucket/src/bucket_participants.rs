use util::button::Button;
use yew::prelude::*;
use yew::format::Json;

use failure::Error;
use yew::services::fetch::Response;

use util::loadable::Loadable;
use util::uploadable::Uploadable;

use common::datatypes::bucket::BucketData;
use common::datatypes::user::UserData;
use wire::user::UserResponse;
use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;

use common::fetch::Networking;
use common::fetch::FetchRequest;
use common::fetch::FetchResponse;

use requests::BucketRequest;



/// A component for approving and rejecting requests to join buckets.
//#[derive(Debug)]
pub struct BucketParticipants {
    users:  Loadable<Vec<UserData>>,
    is_user_bucket_owner: Loadable<bool>,
    bucket_uuid: Option<BucketUuid>,
    remove_user_action: Uploadable<()>,
    networking: Networking,
    link: ComponentLink<BucketParticipants>,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: Loadable<BucketData>

}

//impl Default for BucketParticipants {
//    fn default() -> Self {
//        BucketParticipants {
//            users: Loadable::default(),
//            is_user_bucket_owner: Loadable::default(),
//            bucket_uuid: None,
//            remove_user_action: Uploadable::default()
//        }
//    }
//}

pub enum Msg {
    GetBucketUserData{bucket_uuid: BucketUuid},
    HandleGetBucketUserDataResponse(FetchResponse<Vec<UserResponse>>),
//    BucketUserDataLoaded(Vec<UserData>),
//    BucketUserDataFailed,
    HandleIsUserOwnerResponse(FetchResponse<bool>),
//    SetIsUserOwner(bool),
    RemoveUserFromBucket{user_uuid: UserUuid},
    HandleRemoveUserResponse(FetchResponse<()>),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl BucketParticipants {
    fn get_participants_in_bucket(&mut self, bucket_uuid: BucketUuid) {
        self.networking.fetch(
            BucketRequest::GetUsersInBucket{bucket_uuid},
            |r| Msg::HandleGetBucketUserDataResponse(r),
            &self.link
        );
//        let callback = context.send_back(
//            |response: Response<Json<Result<Vec<UserResponse>, Error>>>| {
//                let (meta, Json(data)) = response.into_parts();
//                println!("META: {:?}, {:?}", meta, data);
//                if meta.status.is_success() {
//                    let new_bucket_users_data: Vec<UserData> = data.map(::wire::convert_vector).unwrap();
//                    Msg::BucketUserDataLoaded(new_bucket_users_data)
//                } else {
//                    Msg::BucketUserDataFailed
//                }
//            },
//        );
//
//        context.make_request_and_set_ft(
//            participants,
//            RequestWrapper::GetUsersInBucket{bucket_uuid},
//            callback,
//        );
    }

    fn determine_if_user_is_owner(&mut self, bucket_uuid: BucketUuid) {
        self.networking.fetch(
            BucketRequest::GetIsUserOwnerOfBucket{bucket_uuid},
            |r| Msg::HandleIsUserOwnerResponse(r),
            &self.link
        );
//        let callback = context.send_back(
//            |response: Response<Json<Result<bool, Error>>>| {
//                let (meta, Json(data)) = response.into_parts();
//                println!("META: {:?}, {:?}", meta, data);
//                if meta.status.is_success() {
//                    Msg::SetIsUserOwner(data.expect("Could not unwrap bool"))
//                } else {
//                    println!("Could not get user->bucket ownership info");
//                    Msg::SetIsUserOwner(false)
//                }
//            },
//        );
//
//        context.make_request_and_set_ft(
//            is_owner,
//            RequestWrapper::GetIsUserOwnerOfBucket{bucket_uuid},
//            callback,
//        );
    }

    fn remove_user_from_bucket(&mut self, bucket_uuid: BucketUuid, user_uuid: UserUuid) {
        self.networking.fetch(
            BucketRequest::RemoveUserFromBucket{bucket_uuid, user_uuid},
            |r| Msg::HandleRemoveUserResponse(r),
            &self.link
        );

//        let bucket_uuid: BucketUuid = bucket_uuid;
//        let callback = context.send_back(
//            move |response: Response<Json<Result<(), Error>>>| {
//                let (meta, Json(data)) = response.into_parts();
//                println!("META: {:?}, {:?}", meta, data);
//                Msg::GetBucketUserData {bucket_uuid} // This is lazy, but just get the whole list again.
//            },
//        );
//
//        context.log(&format!("Removing user from bucket:{}-{}",user_uuid,bucket_uuid));
//
//        context.make_request_and_set_ft(
//            remove_user_action,
//            RequestWrapper::RemoveUserFromBucket{bucket_uuid, user_uuid},
//            callback,
//        );
    }
}


impl Component for BucketParticipants {
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut participants: BucketParticipants = BucketParticipants {
            users: Loadable::default(),
            is_user_bucket_owner: Loadable::default(),
            bucket_uuid: None,
            remove_user_action: Uploadable::default(),
            networking: Networking::new(&link),
            link,
        };

        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            participants.get_participants_in_bucket(bucket_data.uuid); // TODO Possibly don't load this on startup, only do this when opening the pane
            participants.determine_if_user_is_owner(bucket_data.uuid);
            participants.bucket_uuid = Some(bucket_data.uuid);
        }
        participants
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        use self::Msg::*;
        match msg {
            GetBucketUserData {bucket_uuid} => {
                self.networking.fetch(
                    BucketRequest::GetUsersInBucket{bucket_uuid},
                    |r| Msg::HandleGetBucketUserDataResponse(r),
                    &self.link
                );
//                Self::get_participants_in_bucket(bucket_uuid, &mut self.users, context);
            }
            HandleGetBucketUserDataResponse(response) => {
                let response: FetchResponse<Vec<UserData>> = response.map(::wire::convert_vector);
                self.users = Loadable::from_fetch_response(response);
            }
//            BucketUserDataLoaded(bucket_user_data) => {
//                self.users = Loadable::Loaded(bucket_user_data);
//            },
//            BucketUserDataFailed => error!("Failed to get bucket user data"),
            HandleIsUserOwnerResponse(is_owner_response) => {
                self.is_user_bucket_owner = Loadable::from_fetch_response(is_owner_response);
            }
//            SetIsUserOwner(is_owner) => self.is_user_bucket_owner = Loadable::Loaded(is_owner),
            RemoveUserFromBucket{user_uuid} => {
                if let Some(bucket_uuid) = self.bucket_uuid {
//                    Self::remove_user_from_bucket(bucket_uuid, user_uuid, &mut self.remove_user_action, context);
                    self.remove_user_from_bucket(bucket_uuid, user_uuid);
                } else {
                    warn!("Couldn't remove user because bucket id is unknown.")
                }
            }
            HandleRemoveUserResponse(response) => {
                if let FetchResponse::Success(_) = response {
                    if let Some(bucket_uuid) = self.bucket_uuid {
                        self.update(GetBucketUserData{bucket_uuid});
                        return true
                    }
                }
                return false
            }
            NoOp => {return false}
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            self.get_participants_in_bucket(bucket_data.uuid);
//            Self::determine_if_user_is_owner(bucket_data.uuid, &mut self.is_user_bucket_owner, context);
            self.determine_if_user_is_owner(bucket_data.uuid);
            self.bucket_uuid = Some(bucket_data.uuid);
            true
        } else {
            false
        }
    }
}



impl Renderable<BucketParticipants> for BucketParticipants {
    fn view(&self) -> Html<BucketParticipants> {

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
    fn users_owner_view(users: &Vec<UserData>) -> Html<BucketParticipants> {
        Self::users_view(users, true)
    }
    fn users_not_owner_view(users: &Vec<UserData>) -> Html<BucketParticipants> {
        Self::users_view(users, false)
    }

    fn users_view(users: &Vec<UserData>, is_user_owner: bool) -> Html<BucketParticipants> {

        fn user_view(user: &UserData, is_user_owner: bool) -> Html<BucketParticipants> {
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

