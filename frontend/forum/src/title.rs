use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterButton;
use util::loadable::Loadable;
use common::datatypes::forum::ForumData;

use wire::forum::ForumResponse;
use common::fetch::FetchResponse;
use common::fetch::Networking;
use identifiers::forum::ForumUuid;
use requests::ForumRequest;

pub struct ForumTitle {
    /// If empty, then show the list title. If populated, then show the specific forum's title.
    chosen_forum: Loadable<ForumData>,
    networking: Networking,
    link: ComponentLink<ForumTitle>
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForumTitleProps {
    forum_uuid: ForumUuid
}

pub enum Msg {
    HandleGetForumRequest(FetchResponse<ForumData>),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}



impl ForumTitle {
    fn get_forum(&mut self, forum_uuid: ForumUuid) {
        self.networking.fetch(
            &ForumRequest::GetForum{forum_uuid},
            |r: FetchResponse<ForumResponse>| Msg::HandleGetForumRequest(r.map(ForumData::from)),
            &self.link
        );
    }
}

impl Component for ForumTitle {
    type Message = Msg;
    type Properties = ForumTitleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut forum_title = ForumTitle {
            chosen_forum: Loadable::default(),
            networking: Networking::new(&link),
            link
        };

        forum_title.get_forum(props.forum_uuid);

        forum_title
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleGetForumRequest(response) => {
                self.chosen_forum = Loadable::from_fetch_response(response);
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let needs_refresh: bool = {
            if let Loadable::Loaded(ref forum) = self.chosen_forum {
                forum.uuid != props.forum_uuid
            } else {
                true
            }
        };
        if needs_refresh {
            self.get_forum(props.forum_uuid)
        }
        false
    }
}

impl Renderable<ForumTitle> for ForumTitle {
    fn view(&self) -> Html<ForumTitle> {
        fn forum_title(forum_data: &ForumData) -> Html<ForumTitle> {
            html! {
                <div>
                    {&forum_data.title}
                    <RouterButton: text="New Thread", route=route!("forum/{}/create", forum_data.uuid), />
                </div>
            }
        }

        self.chosen_forum.small_view(forum_title)
    }
}

impl Routable for ForumTitle {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if let Some(seg_2) = route.path_segments.get(1) {
            if let Ok(forum_uuid) = ForumUuid::parse_str(&seg_2) {
                Some(
                    ForumTitleProps {forum_uuid}
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
            seg_1.as_str() == "forum"
        } else {
            false
        }
    }
}


pub struct ForumsTitle;
#[derive(Clone, PartialEq, Default)]
pub struct ForumsTitleProps;

impl Component for ForumsTitle {
    type Message = ();
    type Properties = ForumsTitleProps;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ForumsTitle
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<ForumsTitle> for ForumsTitle {
    fn view(&self) -> Html<ForumsTitle> {
        html! {
            <div>
                {"Forums"}
                <RouterButton: text="New Forum", route=route!("forum/create"), />
            </div>
        }
    }
}

impl Routable for ForumsTitle {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if route.path_segments.get(1).is_none() {
            info!("routing forums title");
            Some(ForumsTitleProps)
        } else {
            None
        }
    }
    fn will_try_to_route(route: &Route) -> bool {
        if let Some(seg_1) = route.path_segments.get(0) {
            seg_1.as_str() == "forum"
        } else {
            false
        }
    }
}
