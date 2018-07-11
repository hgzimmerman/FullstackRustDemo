use yew::prelude::*;
use yew_router::prelude::*;
use post_tree::PostTree;
use common::datatypes::post::PostData;
use identifiers::thread::ThreadUuid;
use identifiers::user::UserUuid;
use common::fetch::Networking;
use util::loadable::Loadable;
use post_tree::Props as PostTreeProps;
use common::fetch::FetchResponse;
use requests::ForumRequest;
use yew::services::storage::{StorageService, Area};
use wire::thread::ThreadResponse;

pub struct Thread {
    posts: Loadable<PostTreeProps>,
    thread_uuid: ThreadUuid,
    user_uuid: Option<UserUuid>,
    networking: Networking,
    link: ComponentLink<Thread>
}
#[derive(Clone, PartialEq, Default)]
pub struct ThreadProps {
    thread_uuid: ThreadUuid
}

pub enum Msg {
    HandleGetThreadResponse(FetchResponse<PostData>),
    // TODO, should handle log out to remove the user uuid.2
    NoOp
}
impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl Thread {
    fn get_posts(&mut self) {
        info!("Getting Posts: Thread Component");
        let thread_uuid: ThreadUuid = self.thread_uuid;
        self.networking.fetch(
            ForumRequest::GetThread{thread_uuid},
            |r: FetchResponse<ThreadResponse>| Msg::HandleGetThreadResponse(r.map(|x| PostData::from(x.posts))), // TODO, I throw away a lot of info here
            &self.link
        );
    }
}

impl Component for Thread {
    type Message = Msg;
    type Properties = ThreadProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut storage_service = StorageService::new(Area::Local);

        let mut thread = Thread {
            posts: Loadable::default(),
            thread_uuid: props.thread_uuid,
            user_uuid: ::common::user::user_id(&mut storage_service).ok(),
            networking: Networking::new(&link),
            link
        };

        thread.get_posts();
        thread
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleGetThreadResponse(response) => {
                let response = response.map(|post_data| {
                    PostTreeProps {
                        post: post_data,
                        thread_uuid: self.thread_uuid,
                        user_uuid: self.user_uuid
                    }
                });
                self.posts = Loadable::from_fetch_response(response);
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.thread_uuid != props.thread_uuid {
            self.thread_uuid = props.thread_uuid;
            self.get_posts();
        }
        false
    }
}

impl Renderable<Thread> for Thread {
    fn view(&self) -> Html<Thread> {
        fn posts_view(props: &PostTreeProps) -> Html<Thread> {
            let props = props.clone();
            html! {
                <div>
                    <PostTree: with props ,/>
                </div>
            }
        }

        self.posts.default_view(posts_view)
    }
}

impl Routable for Thread {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        // /forum/forum_uuid/<thread_uuid>
        if let Some(seg_3) = route.path_segments.get(2) {
            if let Ok(thread_uuid) = ThreadUuid::parse_str(&seg_3) {
                Some(
                     ThreadProps{thread_uuid}
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        if let Some(seg_1) = route.path_segments.get(0) {
            seg_1.as_str() == "forum" && route.path_segments.get(2).is_some()
        } else {
            false
        }
    }
}
