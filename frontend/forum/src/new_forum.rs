use common::fetch::FetchResponse;
use common::fetch::Networking;
use requests::ForumRequest;
use util::button::Button;
use util::uploadable::Uploadable;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::router_agent::RouterSenderBase;
use util::input::Input;
use util::input::InputState;
use wire::forum::NewForumRequest;
use wire::forum::ForumResponse;
//use Context;

#[derive(Default, Clone)]
pub struct NewForumData {
    pub title: InputState,
    pub description: InputState
}



pub struct NewForum {
    new_forum: Uploadable<NewForumData>,
    is_user_admin: bool,
    router_sender: RouterSenderBase<()>,
    networking: Networking,
    link: ComponentLink<NewForum>
}

impl NewForum {
    fn send_create_new_forum_request(&mut self) {
        let new_forum_request: NewForumRequest  = NewForumRequest {
            title: self.new_forum.as_ref().title.inner_text(),
            description: self.new_forum.as_ref().description.inner_text()
        };
        self.networking.fetch(
            &ForumRequest::CreateForum(new_forum_request),
            Msg::HandleCreateNewForumResponse,
            &self.link
        );
    }
}


pub enum Msg {
    SendCreateNewForumRequest,
    HandleCreateNewForumResponse(FetchResponse<ForumResponse>), // TODO determine this type
    UpdateTitle(InputState),
    UpdateDescription(InputState),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Props;


impl Component for NewForum {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let mut storage_service = StorageService::new(Area::Local);
        let is_user_admin: bool = ::common::user::user_is_admin(&mut storage_service);

        let route_cb = link.send_back(|_| Msg::NoOp);

        NewForum {
            new_forum: Uploadable::default(),
            is_user_admin,
            router_sender: RouterSenderBase::new(route_cb),
            networking: Networking::new(&link),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendCreateNewForumRequest => {
                self.send_create_new_forum_request();
                false
            }
            Msg::HandleCreateNewForumResponse(response) => {
                match response {
                    FetchResponse::Success(forum_response) => {
                        let forum_uuid = forum_response.uuid;
                        self.router_sender.send(RouterRequest::ChangeRoute(route!("forum/{}", forum_uuid))) // TODO, make this more specific
                    },
                    FetchResponse::Error(_) => {
                        self.new_forum.set_failed("Couldn't create new forum.")
                    },
                    FetchResponse::Started => {
                        self.new_forum.set_uploading()
                    }
                }
                true
            },
            Msg::UpdateTitle(title) => {
                self.new_forum.as_mut().title = title;
                true
            }
            Msg::UpdateDescription(text) => {
                self.new_forum.as_mut().description = text;
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<NewForum> for NewForum {
    fn view(&self) -> Html<Self> {
        self.new_forum.default_view(NewForumData::view)
    }
}

impl Renderable<NewForum> for NewForumData {
    fn view(&self) -> Html<NewForum> {

        html! {
            <div>
                <Input:
                    input_state=&self.title,
                    placeholder="Forum Title",
                    on_change= Msg::UpdateTitle,
                />

                <Input:
                    input_state=&self.description,
                    placeholder="Description",
                    on_change= Msg::UpdateDescription,
                />
                <Button: title="submit", onclick=|_| Msg::SendCreateNewForumRequest, />

            </div>
        }
    }
}
impl Routable for NewForum {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        // /forum/create
        if let Some(seg_2) = route.path_segments.get(1) {
            if seg_2.as_str() == "create" {
                return Some(Props)
            }
        }
        None
    }

    // TODO this is _wrong_ish_
    fn will_try_to_route(route: &Route) -> bool {
        if let Some(seg_1) = route.path_segments.get(0) {
            seg_1.as_str() == "forum" //&& route.path_segments.get(1).is_some()
        } else {
            false
        }
    }
}
