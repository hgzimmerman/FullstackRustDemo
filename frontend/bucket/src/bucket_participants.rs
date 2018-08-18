use util::button::Button;
use yew::prelude::*;


use util::loadable::Loadable;

use common::datatypes::bucket::BucketData;
use common::datatypes::user::UserData;
use wire::user::UserResponse;
use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;

use common::fetch::Networking;
use common::fetch::FetchResponse;

use requests::BucketRequest;



/// A component for approving and rejecting requests to join buckets.
//#[derive(Debug)]
pub struct BucketParticipants {
    users:  Loadable<Vec<UserData>>,
    is_user_bucket_owner: Loadable<bool>,
    bucket_uuid: Option<BucketUuid>,
    networking: Networking,
    link: ComponentLink<BucketParticipants>,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: Loadable<BucketData>

}


pub enum Msg {
    GetBucketUserData{bucket_uuid: BucketUuid},
    HandleGetBucketUserDataResponse(FetchResponse<Vec<UserData>>),
    HandleIsUserOwnerResponse(FetchResponse<bool>),
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
            &BucketRequest::GetUsersInBucket{bucket_uuid},
            |r: FetchResponse<Vec<UserResponse>>| Msg::HandleGetBucketUserDataResponse(r.map(::wire::convert_vector)),
            &self.link
        );
    }

    fn determine_if_user_is_owner(&mut self, bucket_uuid: BucketUuid) {
        self.networking.fetch(
            &BucketRequest::GetIsUserOwnerOfBucket{bucket_uuid},
            Msg::HandleIsUserOwnerResponse,
            &self.link
        );
    }

    fn remove_user_from_bucket(&mut self, bucket_uuid: BucketUuid, user_uuid: UserUuid) {
        self.networking.fetch(
            &BucketRequest::RemoveUserFromBucket{bucket_uuid, user_uuid},
            Msg::HandleRemoveUserResponse,
            &self.link
        );
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
            networking: Networking::new(&link),
            link,
        };

        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            participants.get_participants_in_bucket(bucket_data.uuid);
            participants.determine_if_user_is_owner(bucket_data.uuid);
            participants.bucket_uuid = Some(bucket_data.uuid);
        }
        participants
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        use self::Msg::*;
        match msg {
            GetBucketUserData {bucket_uuid} => {
                self.get_participants_in_bucket(bucket_uuid);
            }
            HandleGetBucketUserDataResponse(response) => {
                self.users = Loadable::from_fetch_response(response);
            }
            HandleIsUserOwnerResponse(is_owner_response) => {
                self.is_user_bucket_owner = Loadable::from_fetch_response(is_owner_response);
            }
            RemoveUserFromBucket{user_uuid} => {
                if let Some(bucket_uuid) = self.bucket_uuid {
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

