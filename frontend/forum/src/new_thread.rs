use common::fetch::FetchResponse;
use common::fetch::Networking;
use datatypes::thread::NewThreadData;
use identifiers::forum::ForumUuid;
use identifiers::user::UserUuid;
use requests::ForumRequest;
use util::button::Button;
use util::markdown::author_markdown_toggle::AuthorMarkdownToggle;
use util::uploadable::Uploadable;
use wire::thread::NewThreadRequest;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::router_agent::RouterSenderBase;
use wire::thread::ThreadResponse;
//use Context;

pub struct NewThread {
    new_thread: Uploadable<NewThreadData>,
    user_uuid: UserUuid,
    forum_uuid: ForumUuid,
    router_sender: RouterSenderBase<()>,
    networking: Networking,
    link: ComponentLink<NewThread>
}

impl NewThread {
    fn send_create_new_thread_request(&mut self) {
        let new_thread_request: NewThreadRequest = self.new_thread
            .as_ref()
            .attach_info(self.forum_uuid, self.user_uuid);
        self.networking.fetch(
            &ForumRequest::CreateThread(new_thread_request),
            Msg::HandleCreateNewThreadResponse,
            &self.link
        );
    }
}


pub enum Msg {
    SendCreateNewThreadRequest,
    HandleCreateNewThreadResponse(FetchResponse<ThreadResponse>),
    UpdatePostContent(String),
    UpdateThreadTitle(String),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Props {
    forum_uuid: ForumUuid,
}


impl Component for NewThread {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let mut storage_service = StorageService::new(Area::Local);
        let user_uuid: UserUuid = ::common::user::user_id(&mut storage_service)
            .expect("This page should not be loadable without being logged in, so the user_uuid should be available");

        let route_cb = link.send_back(|_| Msg::NoOp);

        NewThread {
            new_thread: Uploadable::default(),
            user_uuid,
            forum_uuid: props.forum_uuid,
            router_sender: RouterSenderBase::new(route_cb),
            networking: Networking::new(&link),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendCreateNewThreadRequest => {
                self.send_create_new_thread_request();
                false
            }
            Msg::HandleCreateNewThreadResponse(response) => {
                match response {
                    FetchResponse::Success(thread_response) => {
                        let thread_uuid = thread_response.uuid;
                        let forum_uuid = thread_response.forum_uuid;
                        self.router_sender.send(RouterRequest::ChangeRoute(route!("forum/{}/{}", forum_uuid, thread_uuid)))
                    },
                    FetchResponse::Error(_) => {
                        self.new_thread.set_failed("Couldn't create new thread.")
                    },
                    FetchResponse::Started => {
                        self.new_thread.set_uploading()
                    }
                }
                true
            },
            Msg::UpdateThreadTitle(title) => {
                self.new_thread.as_mut().title = title;
                true
            }
            Msg::UpdatePostContent(text) => {
                self.new_thread.as_mut().post_content = text;
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<NewThread> for NewThread {
    fn view(&self) -> Html<Self> {
        self.new_thread.default_view(NewThreadData::view)
    }
}

impl Renderable<NewThread> for NewThreadData {
    fn view(&self) -> Html<NewThread> {

        return html! {
            <div>
                <input
                    class="form-control",
                //    disabled=self.disabled,
                    placeholder="Thread Title",
                    value=&self.title,
                    oninput=|e| Msg::UpdateThreadTitle(e.value),
//                    onkeypress=|e: KeyData| {
//                        if e.key == "Enter" { Msg::Submit } else {Msg::NoOp}
//                    },
                 />
                 <AuthorMarkdownToggle: callback=|text| Msg::UpdatePostContent(text), />
                 <Button: onclick=|_| Msg::SendCreateNewThreadRequest, />

            </div>
        };
    }
}

impl Routable for NewThread {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        // /forum/forum_uuid/create
        if let Some(seg_2) = route.path_segments.get(1) {
            if let Ok(forum_uuid) = ForumUuid::parse_str(&seg_2) {
                if let Some(seg_3) = route.path_segments.get(2) {
                    if seg_3.as_str() == "create" {
                        return Some(
                             Props{forum_uuid}
                        )
                    }
                }

            }
        };
        None
    }

    // TODO this is _wrong_ish_
    fn will_try_to_route(route: &Route) -> bool {
        if let Some(seg_1) = route.path_segments.get(0) {
            seg_1.as_str() == "forum" && route.path_segments.get(1).is_some() && route.path_segments.get(2).is_some()
        } else {
            false
        }
    }
}
