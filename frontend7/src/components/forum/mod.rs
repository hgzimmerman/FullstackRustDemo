
use yew::services::route::*;
use yew::html::Renderable;
use yew::prelude::*;

use Context;
use Model;
//use forum::forum::Forum;

//use forum::forum_list::ForumList;
use components::forum::thread::new_thread::NewThread;
use components::forum::thread::thread::Thread;

mod forum_list_element;
mod thread;
//pub mod forum_list;
//mod forum;

use util::Loadable;
use util::Either;
use yew::format::Json;
use yew::services::fetch::Response;
use yew::services::fetch::FetchTask;
use failure::Error;
use Route;

use wire::thread::MinimalThreadResponse;
use wire::forum::ForumResponse;
use wire::thread::ThreadResponse;
use context::networking::RequestWrapper;

use datatypes::thread::ThreadData;
use datatypes::thread::MinimalThreadData;
use datatypes::forum::ForumData;
use datatypes::thread::NewThreadData;

use components::forum::thread::thread_list_element::ThreadListElement;
use components::forum::forum_list_element::ForumListElement;


#[derive(Debug, PartialEq, Clone)]
pub enum ForumRoute {
    ForumList,
    Forum{forum_id: i32},
    Thread {
        forum_id: i32,
        thread_id: i32
    },
    CreateThread {
        forum_id: i32
    }
}

impl Default for ForumRoute {
    fn default() -> Self {
        ForumRoute::ForumList
    }
}

impl ForumRoute {
    fn get_forum_id(&self) -> Option<i32> {
        match self {
            ForumRoute::Forum{forum_id} => Some(forum_id.clone()),
            ForumRoute::Thread {forum_id, ..} => Some(forum_id.clone()),
            ForumRoute::CreateThread {forum_id} => Some(forum_id.clone()),
            _ => None
        }
    }
}

impl Router for ForumRoute {
    fn to_route(&self) -> RouteInfo {
        match *self {
            ForumRoute::ForumList => RouteInfo::parse("/").unwrap(),
            ForumRoute::Forum{forum_id} => {
                RouteInfo::parse(&format!("/{}", forum_id)).unwrap()
            }
            ForumRoute::Thread {forum_id, thread_id} => {
                RouteInfo::parse(&format!("/{}/{}", forum_id, thread_id)).unwrap()
            }
            ForumRoute::CreateThread {forum_id} => {
                RouteInfo::parse(&format!("/{}/create", forum_id)).unwrap()
            }
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        if let Some(RouteSection::Node { segment }) = route.next() {
            if let Ok(forum_id) = segment.parse::<i32>() {
                if let Some(RouteSection::Node {segment}) = route.next() {
                    if &segment == "create" {
                        Some(ForumRoute::CreateThread {forum_id})
                    } else if let Ok(thread_id) = segment.parse::<i32>() {
                        Some(ForumRoute::Thread {forum_id, thread_id})
                    } else {
                        None
                    }
                } else {
                    Some(ForumRoute::Forum{forum_id})
                }
            } else {
                Some(ForumRoute::ForumList) //TODO not sure about either this one or the one below
            }
        } else {
            Some(ForumRoute::ForumList)
        }
    }
}

pub enum Msg {
    ForumsReady(Vec<ForumData>),
    ForumsFailed,
    ForumReady(ForumData),
    ForumLoading(FetchTask),
    ForumFailed,
    ThreadsReady(Vec<MinimalThreadData>),
    ThreadsLoading(FetchTask),
    ThreadsFailed,
    ThreadReady(ThreadData),
    ThreadFailed,
    ThreadLoading(FetchTask),
    SetCreateThread,
    SetThread{thread_id: i32},
    SetForum{forum_data: ForumData}
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Props {
    pub route: ForumRoute
}

#[derive(Default)]
pub struct ForumModel {
    route: ForumRoute,
    forums_or_selected_forum: Either<Loadable<Vec<ForumData>>, Loadable<ForumData>>,
    thread_list: Loadable<Vec<MinimalThreadData>>,
    thread: Either<Loadable<ThreadData>, NewThreadData>
}


impl ForumModel {
    fn get_forum_list(context: &mut Env<Context, Self>) -> Either<Loadable<Vec<ForumData>>, Loadable<ForumData>> {
                let callback = context.send_back(
            |response: Response<Json<Result<Vec<ForumResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);

                if meta.status.is_success() {
                    let forum_data_list: Vec<ForumData> = data.expect("Forum Data invalid")
                        .into_iter()
                        .map(ForumData::from)
                        .collect();

                    Msg::ForumsReady(forum_data_list)
                } else {
                    Msg::ForumsFailed
                }
            },
        );

        let forums_task = context.make_request(RequestWrapper::GetForums, callback);
        if let Ok(ft) = forums_task {
            Either::Left(Loadable::Loading(ft))
        } else {
            Either::Left(Loadable::Failed(Some("Couldn't make request".into())))
        }
    }

    fn get_forum(forum_id: i32, context: &mut Env<Context, Self>) -> Either<Loadable<Vec<ForumData>>, Loadable<ForumData>> {
        let forum_callback = context.send_back(
            |response: Response<Json<Result<ForumResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::ForumReady(data.unwrap().into())
                } else {
                    Msg::ForumFailed
                }
            },
        );

        let forum_task = context.make_request(
            RequestWrapper::GetForum {
                forum_id,
            },
            forum_callback,
        );
        if let Ok(ft) = forum_task {
            Either::Right(Loadable::Loading(ft))
        } else {
            Either::Right(Loadable::Failed(Some("Couldn't make request".into())))
        }

    }

    fn get_threads(forum_id: i32, context: &mut Env<Context, Self>) -> Loadable<Vec<MinimalThreadData>> {
        let threads_callback = context.send_back(
            |response: Response<Json<Result<Vec<MinimalThreadResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::ThreadsReady(
                        data.unwrap()
                            .into_iter()
                            .map(MinimalThreadData::from)
                            .collect(),
                    )
                } else {
                    Msg::ThreadsFailed
                }
            },
        );

        let threads_task = context.make_request(
            RequestWrapper::GetThreads {
                forum_id,
                page_index: 1,
            },
            threads_callback,
        );

        if let Ok(ft) = threads_task {
            Loadable::Loading(ft)
        } else {
            Loadable::Failed(Some("Couldn't make request".into()))
        }
    }

    fn get_thread(thread_id: i32, context: &mut Env<Context, Self>) -> Loadable<ThreadData> {
        let callback = context.send_back(
            |response: Response<Json<Result<ThreadResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
//                context.log(&format!("META: {:?}, {:?}", meta, data));
                if meta.status.is_success() {
                    Msg::ThreadReady(data.unwrap().into())
                } else {
                    Msg::ThreadFailed
                }
            },
        );

        let thread_task = context.make_request(
            RequestWrapper::GetThread {
                thread_id,
            },
            callback,
        );
        if let Ok(ft) = thread_task {
            Loadable::Loading(ft)
        } else {
            Loadable::Failed(Some("Couldn't make request".into()))
        }
    }


    fn set_self(&mut self, other: ForumModel) {
        self.thread_list = other.thread_list;
        self.thread = other.thread;
        self.forums_or_selected_forum = other.forums_or_selected_forum;
        self.route = other.route;
    }
}


impl Component<Context> for ForumModel {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        match props.route {
            ForumRoute::ForumList => {
                let forums_or_selected_forum = Self::get_forum_list(context);
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    ..Default::default()
                }
            }
            ForumRoute::Forum { forum_id } => {
                let forums_or_selected_forum = Self::get_forum(forum_id, context);
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    ..Default::default()
                }
            }
            ForumRoute::Thread { forum_id, thread_id } => {
                let forums_or_selected_forum = Self::get_forum(forum_id, context);
                let thread_list = Self::get_threads(forum_id, context); // TODO add a way of marking the threads in the list as being selected if they have the same id as the "thread" value below. Do this in the Update method.
                let thread = Self::get_thread(thread_id, context);
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    thread_list,
                    thread: Either::Left(thread)
                }
            }
            ForumRoute::CreateThread { forum_id } => {
                let forums_or_selected_forum = Self::get_forum(forum_id, context);
                let thread_list = Self::get_threads(forum_id, context);
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    thread_list,
                    thread: Either::Right(NewThreadData::default())
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::ForumsReady(forums) => self.forums_or_selected_forum = Either::Left(Loadable::Loaded(forums)),
            Msg::ForumsFailed => self.forums_or_selected_forum = Either::Left(Loadable::Failed(None)),
            Msg::ForumReady(forum) => self.forums_or_selected_forum = Either::Right(Loadable::Loaded(forum)),
            Msg::ForumLoading(ft) => self.forums_or_selected_forum = Either::Right(Loadable::Loading(ft)),
            Msg::ForumFailed => self.forums_or_selected_forum = Either::Right(Loadable::Failed(None)),
            Msg::ThreadsReady(threads) => self.thread_list = Loadable::Loaded(threads),
            Msg::ThreadsLoading(ft) => self.thread_list = Loadable::Loading(ft),
            Msg::ThreadsFailed => self.thread_list = Loadable::Failed(None),
            Msg::ThreadReady(thread) => {
                context.log("Thread ready");

                self.thread = Either::Left(Loadable::Loaded(thread))
            },
            Msg::ThreadFailed => self.thread = Either::Left(Loadable::Failed(None)),
            Msg::ThreadLoading(ft) => self.thread = Either::Left(Loadable::Loading(ft)),
            Msg::SetCreateThread => {
                if let Some(forum_id) = self.route.get_forum_id() {
                let route = ForumRoute::CreateThread {forum_id};
                context.routing.set_route(Route::Forums(route));
                }

//                self.thread = Either::Right(Default::default())
            },
            Msg::SetThread {thread_id} => {
                if let Some(forum_id) = self.route.get_forum_id() {
                    let route = ForumRoute::Thread { forum_id, thread_id};
                    context.routing.set_route(Route::Forums(route.clone()));
                }
                self.thread = Either::Left(Self::get_thread(thread_id, context))
            },
            Msg::SetForum {forum_data} => {
                let route = ForumRoute::Forum { forum_id: forum_data.id.clone() };

//                self.forums_or_selected_forum = Either::Right(Loadable::Loaded(forum_data));
                context.routing.set_route(Route::Forums(route));

            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        context.log(&format!("Forum: change() called: {:?}", props.route));


        let cloned_route = props.route.clone();
        let should_render = match props.route.clone() {
            ForumRoute::ForumList => {
                let forums_or_selected_forum = Self::get_forum_list(context);
                let new_state = ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    ..Default::default()
                };
                self.set_self(new_state);
                true
            }
            ForumRoute::Forum { forum_id } => {
                let mut refresh_forums_needed = true;
                if let Either::Right(Loadable::Loaded(ref forum_data)) = self.forums_or_selected_forum {
                    if forum_id == forum_data.id {
                        refresh_forums_needed = false;
                    }
                }

                let forums_or_selected_forum = if refresh_forums_needed {
                    Self::get_forum(forum_id, context)
                } else {
                    self.forums_or_selected_forum.clone()
                };
                let new_state = ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    thread_list: Self::get_threads(forum_id, context),
                    ..Default::default()
                };
                self.set_self(new_state);
                context.log(&format!("Setting forum- route: {:?}", self.route));
                true
            }
            ForumRoute::Thread { forum_id, thread_id } => {
                context.log("Thread (new)");
                match self.route {
                    ForumRoute::ForumList => {
                        let forums_or_selected_forum = Self::get_forum_list(context);
                        let new_state = ForumModel {
                            route: props.route,
                            forums_or_selected_forum,
                            ..Default::default()
                        };
                        self.set_self(new_state);
                        true
                    }
                    ForumRoute::Forum {forum_id: old_forum_id} => {
                        context.log("forum");
                        if forum_id != old_forum_id {
                            self.forums_or_selected_forum = Self::get_forum(forum_id, context);
                            self.thread_list = Self::get_threads(forum_id, context); // TODO add a way of marking the threads in the list as being selected if they have the same id as the "thread" value below. Do this in the Update method.
                            true
                        } else {
                            false
                        }
                    }
                    ForumRoute::Thread { forum_id: old_forum_id, thread_id: old_thread_id } => {
                        context.log("Thread (old)");
                        let mut should_render: ShouldRender = false;
                        if forum_id != old_forum_id {
                            self.forums_or_selected_forum = Self::get_forum(forum_id, context);
                            self.thread_list = Self::get_threads(forum_id, context); // TODO add a way of marking the threads in the list as being selected if they have the same id as the "thread" value below. Do this in the Update method.
                            should_render = true;
                        }
                        context.log(&format!("old: {}, new: {}", old_thread_id, thread_id));
                        if thread_id != old_thread_id {
                            self.thread = Either::Left(Self::get_thread(thread_id, context));
                            should_render = true;
                        }
                        should_render
                    }
                    ForumRoute::CreateThread { forum_id: old_forum_id } => {
                        if forum_id != old_forum_id {
                            self.forums_or_selected_forum = Self::get_forum(forum_id, context);
                            self.thread_list = Self::get_threads(forum_id, context); // TODO add a way of marking the threads in the list as being selected if they have the same id as the "thread" value below. Do this in the Update method.
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            ForumRoute::CreateThread { forum_id } => {
                let mut refresh_forums_needed = true;
                if let Either::Right(Loadable::Loaded(ref forum_data)) = self.forums_or_selected_forum {
                    if forum_id == forum_data.id {
                        refresh_forums_needed = false;
                    }
                }


                // TODO this problem _may_ be problematic due to invalidation of loading loadables when they are cloned
                let forums_or_selected_forum = if refresh_forums_needed {
                    Self::get_forum(forum_id, context)
                } else {
                    self.forums_or_selected_forum.clone()
                };
                let thread_list = if refresh_forums_needed {
                    Self::get_threads(forum_id, context)
                } else {
                    self.thread_list.clone()
                };

                let new_state = ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    thread_list,
                    thread: Either::Right(NewThreadData::default())
                };
                self.set_self(new_state);
                true
            }
        };
        self.route = cloned_route; // Set this here, in case it was forgotten earlier.
        should_render
    }
}
impl Renderable<Context, ForumModel> for ForumModel {
    fn view(&self) -> Html<Context, ForumModel> {

        fn forum_element_fn(x: &ForumData) -> Html<Context, ForumModel> {
            html! {
                <ForumListElement: forum_data=x, callback=|forum_data: ForumData| Msg::SetForum{forum_data},/>
            }
        };
        fn forum_list_fn(forums: &Vec<ForumData>) -> Html<Context, ForumModel> {
            html! {
                <ul class=("forum-list"),>
                    { for forums.iter().map(forum_element_fn) }
                </ul>
            }
        };

        fn thread_element_fn(x: &MinimalThreadData) -> Html<Context, ForumModel> {
            html! {
                <ThreadListElement: thread_data=x, callback=|td: MinimalThreadData| Msg::SetThread{thread_id: td.id}, />
            }
        };
        fn thread_list_fn(threads: &Vec<MinimalThreadData>) -> Html<Context, ForumModel> {
            html! {
                <ul class=("forum-list"),>
                    { for threads.iter().map(thread_element_fn) }
                </ul>
            }
        };

        fn thread_fn(thread: &ThreadData) -> Html<Context, ForumModel> {
            html! {
                <>
                   <Thread: thread_data=thread, />
                </>
            }
        }
        fn new_thread_fn(new_thread: &NewThreadData) -> Html<Context, ForumModel> {
            html! {
                <>
                    <NewThread: new_thread=new_thread, />
                </>
            }
        }

        fn forum_title(forum_data: &ForumData) -> Html<Context,ForumModel> {
            html! {
                <>
                    {&forum_data.title}
                </>
            }
        }

        match self.forums_or_selected_forum {
            Either::Left(ref forums) => html! {
                <div>
                    <div>
                        {forums.default_view(forum_list_fn)}
                    </div>
                </div>
            },
            Either::Right(ref selected_forum) =>  html!{
                <div class=("flexbox-vert","full-height"),>
                    <div class="forum-title",>
                        {selected_forum.default_view(forum_title)}
                    </div>
                    <div class=("flexbox-horiz", "full-height"), > // Horizontal boi
                        <div class=("vertical-expand", "list-background", "forum-list-width"),> // Vertical boi 1
                           {self.thread_list.default_view(thread_list_fn)}
                        </div>
                        <div class=("vertical-expand", "full-width" ),> // Vertical boi 2
                            {
                                match self.thread {
                                    Either::Left(ref thread) => thread.default_view(thread_fn),
                                    Either::Right(ref new_thread) => new_thread_fn(new_thread)
                                }
                            }
                        </div>
                    </div>
                </div>
            }
        }
    }
}


