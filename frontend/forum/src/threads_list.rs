use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterLink;
use common::fetch::Networking;
use util::loadable::Loadable;
use requests::ForumRequest;
use common::fetch::FetchResponse;
use identifiers::forum::ForumUuid;
use identifiers::thread::ThreadUuid;

use common::datatypes::thread::SelectableMinimalThreadData;
use common::datatypes::thread::MinimalThreadData;
use wire::thread::MinimalThreadResponse;

pub struct ThreadsList {
    threads: Loadable<Vec<SelectableMinimalThreadData>>,
    forum_uuid: ForumUuid,
    /// The thread uuid isn't always present, but should be used to indicate if a thread is selected.
    chosen_thread_uuid: Option<ThreadUuid>,
    networking: Networking,
    link: ComponentLink<ThreadsList>
}

#[derive(Clone, PartialEq, Default)]
pub struct ThreadsListProps {
    forum_uuid: ForumUuid,
    chosen_thread_uuid: Option<ThreadUuid>,
    page_index: Option<usize>
}

pub enum Msg {
    HandleGetThreadsResponse(FetchResponse<Vec<MinimalThreadData>>),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}


impl Component for ThreadsList {
    type Message = Msg;
    type Properties = ThreadsListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut threads_list = ThreadsList {
            threads: Loadable::default(),
            forum_uuid: props.forum_uuid,
            chosen_thread_uuid: props.chosen_thread_uuid,
            networking: Networking::new(&link),
            link
        };

        let forum_uuid = props.forum_uuid;
        let page_index: usize = props.page_index.unwrap_or(1);


        threads_list.networking.fetch(
            &ForumRequest::GetThreads{forum_uuid, page_index},
            |r: FetchResponse<Vec<MinimalThreadResponse>>| Msg::HandleGetThreadsResponse(r.map(
                |x: Vec<MinimalThreadResponse>| {
                    x.into_iter()
                        .map(MinimalThreadData::from)
                        .collect()
                }
            )),
            &threads_list.link
        );

        threads_list
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleGetThreadsResponse(response) => {
                let response: FetchResponse<Vec<SelectableMinimalThreadData>> = response.map(|x| {
                    x.into_iter()
                        .map(|y|  SelectableMinimalThreadData::new(y, self.forum_uuid, self.chosen_thread_uuid))
                        .collect()
                });
                self.threads = Loadable::from_fetch_response(response);
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.forum_uuid = props.forum_uuid;
        self.chosen_thread_uuid = props.chosen_thread_uuid;

        // Set the selected boolean if the thread uuid of the list item matches the url's thread uuid.
        if let Some(thread_uuid) = self.chosen_thread_uuid {
            if let Some(threads) = self.threads.as_mut_option() {
                threads
                    .iter_mut()
                    .for_each(|thread| thread.is_selected = thread.minimal_thread_data.uuid == thread_uuid )
            }
        }


        true
    }
}

impl Renderable<ThreadsList> for ThreadsList {
    fn view(&self) -> Html<ThreadsList> {
        fn thread_list_fn(threads: &Vec<SelectableMinimalThreadData>) -> Html<ThreadsList> {
            html! {
                <ul class=("forum-list"),>
                    { for threads.iter().map(SelectableMinimalThreadData::view) }
                </ul>
            }
        };

        self.threads.default_view(thread_list_fn)
    }
}



impl Renderable<ThreadsList> for SelectableMinimalThreadData {
   fn view(&self) -> Html<ThreadsList> {

       fn element_internals(minimal_thread_data: &MinimalThreadData, forum_uuid: &ForumUuid) -> Html<ThreadsList> {
           html! {
               <>
                   <div>
                        <RouterLink: text=&minimal_thread_data.title, route=route!("forum/{}/{}", forum_uuid, minimal_thread_data.uuid), />
                   </div>
                   <div>
                        {format!("By: {}", minimal_thread_data.author.display_name)}
                   </div>
               </>
           }

       }
       if !self.is_selected {
           html! {
                <li class="forum-list-element",>
                    {element_internals(&self.minimal_thread_data, &self.forum_uuid)}
                </li>
           }
       } else {
           html! {
                <li class=("forum-list-element","list-element-selected"),>
                    {element_internals(&self.minimal_thread_data, &self.forum_uuid)}
                </li>
           }
       }

   }
}

impl Routable for ThreadsList {
    fn resolve_props(route: &Route) -> Option<<Self as Component>::Properties> {
        if let Some(seg_2) = route.path_segments.get(1) {
            if let Ok(forum_uuid) = ForumUuid::parse_str(&seg_2) {

                let chosen_thread_uuid: Option<ThreadUuid> =
                    route.path_segments
                    .get(2)
                    .and_then(|x|ThreadUuid::parse_str(&x).ok());
                return Some(
                     ThreadsListProps{forum_uuid, chosen_thread_uuid, page_index: Some(1)}
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