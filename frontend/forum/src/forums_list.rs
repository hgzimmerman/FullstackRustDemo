use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterLink;
use common::fetch::Networking;
use util::loadable::Loadable;
use datatypes::forum::ForumData;
use requests::ForumRequest;
use wire::forum::ForumResponse;
use common::fetch::FetchResponse;


pub struct ForumsList {
    forums: Loadable<Vec<ForumData>>,
    networking: Networking,
    link: ComponentLink<ForumsList>
}

#[derive(Clone, PartialEq, Default)]
pub struct ForumsListProps; // TODO possibly add pagination

pub enum Msg {
    HandleGetForumsListResponse(FetchResponse<Vec<ForumData>>),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}


impl Component for ForumsList {
    type Message = Msg;
    type Properties = ForumsListProps;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut forums_list = ForumsList {
            forums: Loadable::default(),
            networking: Networking::new(&link),
            link
        };

        forums_list.networking.fetch(
            ForumRequest::GetForums,
            |r: FetchResponse<Vec<ForumResponse>>| Msg::HandleGetForumsListResponse(r.map(
                |x: Vec<ForumResponse>| {
                    x.into_iter()
                        .map(ForumData::from)
                        .collect()
                }
            )),
            &forums_list.link
        );

        forums_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleGetForumsListResponse(response) => {
                self.forums = Loadable::from_fetch_response(response);
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<ForumsList> for ForumsList {
    fn view(&self) -> Html<ForumsList> {
        fn forum_list_fn(forums: &Vec<ForumData>) -> Html<ForumsList> {
            html! {
                <ul class=("forum-list"),>
                    { for forums.iter().map(ForumData::view) }
                </ul>
            }
        };
        self.forums.default_view(forum_list_fn)
    }
}


impl Renderable<ForumsList> for ForumData {
   fn view(&self) -> Html<ForumsList> {
        html! {
            <li class="forum-list-element",>
                <div>
                    <RouterLink: text=&self.title, route=Route::parse(&format!("forum/{}",self.uuid)), />
                </div>
                <div>
                    {&self.description}
                </div>
            </li>
        }
   }
}
impl Routable for ForumsList {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if let None = route.path_segments.get(1) {
            Some(ForumsListProps)
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
