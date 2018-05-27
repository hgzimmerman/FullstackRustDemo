use context::datatypes::bucket::BucketUsersData;
use Context;
use util::button::Button;
use yew::prelude::*;
use yew::format::Json;

use failure::Error;
use wire::bucket::BucketUsersResponse;
use yew::services::fetch::Response;

use util::loadable::Loadable;

use context::datatypes::bucket::BucketData;
use context::networking::RequestWrapper;
use context::datatypes::user::UserData;
use wire::user::UserResponse;




/// A component for approving and rejecting requests to join buckets.
#[derive(Debug, Clone)]
pub struct BucketParticipants {
    users:  Loadable<Vec<UserData>>,
    is_open: bool,
    is_user_bucket_owner: Loadable<bool>
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: Loadable<BucketData>

}

impl Default for BucketParticipants {
    fn default() -> Self {
        BucketParticipants {
            users: Loadable::default(),
            is_open: false,
            is_user_bucket_owner: Loadable::default()
        }
    }
}

pub enum Msg {
    BucketUserDataLoaded(Vec<UserData>),
    BucketUserDataFailed,
    TogglePane
}

impl BucketParticipants {
    fn get_participants_in_bucket(bucket_id: i32, participants: &mut Loadable<Vec<UserData>>, context: &mut Env<Context, Self>) {
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

        context.make_logoutable_request(
            participants,
            RequestWrapper::GetUsersInBucket{bucket_id},
            callback,
        );
    }
    fn determine_if_user_is_owner(bucket_id: i32, is_owner: &mut Loadable<bool>, context: &mut Env<Context, Self>) {
         unimplemented!()
    }
}


impl Component<Context> for BucketParticipants {
    type Msg = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut participants: BucketParticipants = BucketParticipants::default();
        if let Loadable::Loaded(bucket_data) = props.bucket_data {
             Self::get_participants_in_bucket(bucket_data.id, &mut participants.users, context) // TODO Possibly don't load this on startup, only do this when opening the pane
        }
        participants
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            BucketUserDataLoaded(bucket_user_data) => {
                self.users = Loadable::Loaded(bucket_user_data)
            },
            BucketUserDataFailed => context.log("Failed to get bucket user data"),
            TogglePane => self.is_open = !self.is_open
        }
        true
    }


    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        if let Loadable::Loaded(bucket_data) = props.bucket_data {
            Self::get_participants_in_bucket(bucket_data.id, &mut self.users, context);

            true
        } else {
            false
        }
    }
}



impl Renderable<Context, BucketParticipants> for BucketParticipants {
    fn view(&self) -> Html<Context, BucketParticipants> {

        let pane = if self.is_open {
            html! {
                <div style="position: absolute; top: 40px; width: 200px; left: -110px; border: 1px solid black; min-height: 200px; background-color: white",>
                    {
                        if let Loadable::Loaded(is_owner) = self.is_user_bucket_owner {
                            if is_owner {
                                self.users.default_view( Self::users_owner_view)
                            }  else {
                                self.users.default_view( Self::users_not_owner_view)
                            }
                        }
                        else {
                            self.users.default_view( Self::users_not_owner_view)
                        }
                    }
                </div>
            }
        } else {
            ::util::wrappers::empty_vdom_node()
        };

        html! {
            <div style="position: relative",>
                <Button: title="Participants", onclick=|_| Msg::TogglePane, />
                {pane}
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
            let delete_button = if !is_user_owner {
                html! {
                    <Button: title="Remove" ,/>
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

