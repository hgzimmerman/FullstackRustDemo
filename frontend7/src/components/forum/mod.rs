
use yew::services::route::*;
use yew::html::Renderable;
use yew::prelude::*;

use Context;
//use Model;
//use forum::forum::Forum;
use components::forum::new_thread::NewThread;
use components::forum::post_tree::PostTree;
use components::button::Button;


mod post_tree;
mod list_elements;
mod new_thread;
//pub mod forum_list;
//mod forum;

use util::Loadable;
use util::Uploadable;

//use util::Either;
use yew::format::Json;
use yew::services::fetch::Response;
//use yew::services::fetch::FetchTask;
use failure::Error;
use Route;

use wire::thread::MinimalThreadResponse;
use wire::thread::NewThreadRequest;
use wire::forum::ForumResponse;
use wire::thread::ThreadResponse;
use context::networking::RequestWrapper;

use datatypes::thread::ThreadData;
use datatypes::thread::MinimalThreadData;
use datatypes::forum::ForumData;
use datatypes::thread::NewThreadData;
use datatypes::thread::SelectableMinimalThreadData;


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
    ForumFailed,
    ThreadsReady(Vec<SelectableMinimalThreadData>),
    ThreadsFailed,
    ThreadReady(ThreadData),
    NewThreadReady(ThreadData),
    ThreadFailed,
    SetCreateThread,
    SetThread{thread_id: i32},
    SetForum{forum_data: ForumData},
    PostNewThread{new_thread: NewThreadData}
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Props {
    pub route: ForumRoute
}

#[derive(Debug, Clone)]
pub enum ForumsOrForum {
    Forums(Loadable<Vec<ForumData>>),
    Forum { forum: Loadable<ForumData>, threads: Loadable<Vec<SelectableMinimalThreadData>> }
}
pub enum ThreadOrNewThread {
    Thread(Loadable<ThreadData>),
    NewThread(Uploadable<NewThreadData>)
}


impl Default for ForumsOrForum {
    fn default() -> Self {
        ForumsOrForum::Forums(Default::default())
    }
}
impl Default for ThreadOrNewThread {
    fn default() -> Self {
        ThreadOrNewThread::Thread(Default::default())
    }
}

#[derive(Default)]
pub struct ForumModel {
    route: ForumRoute, // TODO, Should I need to store this? I should be able to reconstruct it from the internal types
    forums_or_selected_forum: ForumsOrForum,
    thread: ThreadOrNewThread
}


impl ForumModel {
    fn get_forum_list(context: &mut Env<Context, Self>) -> Loadable<Vec<ForumData>> {
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
            Loadable::Loading(ft)
        } else {
            Loadable::Failed(Some("Couldn't make request".into()))
        }
    }

    fn get_forum(forum_id: i32, context: &mut Env<Context, Self>) -> Loadable<ForumData> {
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
            Loadable::Loading(ft)
        } else {
            Loadable::Failed(Some("Couldn't make request".into()))
        }

    }

    fn get_threads(forum_id: i32, context: &mut Env<Context, Self>) -> Loadable<Vec<SelectableMinimalThreadData>> {
        let threads_callback = context.send_back(
            |response: Response<Json<Result<Vec<MinimalThreadResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::ThreadsReady(
                        data.unwrap()
                            .into_iter()
                            .map(MinimalThreadData::from)
                            .map(SelectableMinimalThreadData::from)
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

    fn upload_new_thread(new_thread: NewThreadData, forum_id: i32, context: &mut Env<Context, Self>) -> Uploadable<NewThreadData>
    {
        if let Ok(user_id) = context.user_id() {

            let callback = context.send_back(
                |response: Response<Json<Result<ThreadResponse, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);

                    if meta.status.is_success() {
                        Msg::NewThreadReady(data.expect("NewThread data is malformed").into())
                    } else {
                        Msg::ThreadFailed
                    }
                },
            );

            let new_thread_request: NewThreadRequest = new_thread.attach_info(forum_id, user_id);

            let task = context.make_request(
                RequestWrapper::CreateThread(
                    new_thread_request,
                ),
                callback,
            );
            if let Ok(ft) = task {
                Uploadable::Uploading(new_thread.clone(), ft)
            } else {
                Uploadable::NotUploaded(new_thread.clone())
            }
        } else {
            Uploadable::NotUploaded(new_thread.clone())
        }
    }


    fn select_thread_in_list(&mut self) {
        if let ForumsOrForum::Forum{threads: Loadable::Loaded(ref mut thread_list), .. } = self.forums_or_selected_forum {
            if let ThreadOrNewThread::Thread(Loadable::Loaded(ref mut selected_thread)) = self.thread {
                 *thread_list = thread_list
                     .iter()
                     .cloned()
                     .map(|x: SelectableMinimalThreadData| {
                         let mut replacement = x.clone();
                         if x.minimal_thread_data.id == selected_thread.id {
                             replacement.is_selected = true;
                         } else {
                             replacement.is_selected = false;
                         }
                         replacement
                     })
                     .collect();
            } else {
                *thread_list = thread_list
                    .iter()
                    .cloned()
                    .map(|x: SelectableMinimalThreadData| {
                        SelectableMinimalThreadData {
                            is_selected: false,
                            ..x
                        }
                    })
                    .collect()
            }
        }
    }
}


impl Component<Context> for ForumModel {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        match props.route {
            ForumRoute::ForumList => {
                let forums_or_selected_forum = ForumsOrForum::Forums(Self::get_forum_list(context));
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    ..Default::default()
                }
            }
            ForumRoute::Forum { forum_id } => {
                let forums_or_selected_forum: ForumsOrForum = ForumsOrForum::Forum {
                    forum: Self::get_forum(forum_id, context),
                    threads: Self::get_threads(forum_id, context)
                };
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    ..Default::default()
                }
            }
            ForumRoute::Thread { forum_id, thread_id } => {
                let forums_or_selected_forum: ForumsOrForum = ForumsOrForum::Forum {
                    forum: Self::get_forum(forum_id, context),
                    threads: Self::get_threads(forum_id, context)
                };
                let thread = ThreadOrNewThread::Thread(Self::get_thread(thread_id, context));
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    thread
                }
            }
            ForumRoute::CreateThread { forum_id } => {
                let forums_or_selected_forum: ForumsOrForum = ForumsOrForum::Forum {
                    forum: Self::get_forum(forum_id, context),
                    threads: Self::get_threads(forum_id, context)
                };
                ForumModel {
                    route: props.route,
                    forums_or_selected_forum,
                    thread: ThreadOrNewThread::NewThread(Uploadable::default())
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::ForumsReady(forums) => self.forums_or_selected_forum = ForumsOrForum::Forums(Loadable::Loaded(forums)),
            Msg::ForumsFailed => self.forums_or_selected_forum = ForumsOrForum::Forums(Loadable::Failed(None)),
            Msg::ForumReady(forum) => {
                if let ForumsOrForum::Forum{forum: ref mut existing_forum, ..} = self.forums_or_selected_forum {
                   *existing_forum = Loadable::Loaded(forum);
                } else {
                    self.forums_or_selected_forum = ForumsOrForum::Forum {
                        forum: Loadable::Loaded(forum),
                        threads: Loadable::Unloaded
                    }
                }
            },
            Msg::ForumFailed => {
                if let ForumsOrForum::Forum{ ref mut forum, ..} = self.forums_or_selected_forum {
                   *forum = Loadable::Failed(None);
                } else {
                    self.forums_or_selected_forum = ForumsOrForum::Forum {
                        forum: Loadable::Failed(None),
                        threads: Loadable::Unloaded
                    }
                }
            },
            Msg::ThreadsReady(threads) => {
                if let ForumsOrForum::Forum{ threads: ref mut existing_threads, ..} = self.forums_or_selected_forum {
                   *existing_threads = Loadable::Loaded(threads);
                } else {
                    self.forums_or_selected_forum = ForumsOrForum::Forum {
                        forum: Loadable::Unloaded,
                        threads: Loadable::Loaded(threads)
                    }
                }
            },
            Msg::ThreadsFailed => {
                if let ForumsOrForum::Forum{ ref mut threads, ..} = self.forums_or_selected_forum {
                   *threads = Loadable::Failed(None)
                } else {
                    self.forums_or_selected_forum = ForumsOrForum::Forum {
                        forum: Loadable::Unloaded,
                        threads: Loadable::Failed(None)
                    }
                }
            },
            Msg::ThreadReady(thread) => {
                let route = ForumRoute::Thread { forum_id: thread.forum_id, thread_id: thread.id};
                context.routing.set_route(Route::Forums(route.clone()));
                self.thread = ThreadOrNewThread::Thread(Loadable::Loaded(thread));
                self.select_thread_in_list();
            },
            Msg::NewThreadReady(thread) => {
                let route = ForumRoute::Thread { forum_id: thread.forum_id.clone(), thread_id: thread.id};
                context.routing.set_route(Route::Forums(route.clone()));

                if let ForumsOrForum::Forum {threads: ref mut existing_thread, ..} = self.forums_or_selected_forum {
                    *existing_thread = Self::get_threads(thread.forum_id, context);
                } else {
                    let f = Self::get_forum(thread.forum_id, context);
                    let t = Self::get_threads(thread.forum_id, context);
                    self.forums_or_selected_forum = ForumsOrForum::Forum {forum: f, threads: t}
                }

                self.thread = ThreadOrNewThread::Thread(Loadable::Loaded(thread));
                self.select_thread_in_list();
            }
            Msg::ThreadFailed => {
                self.thread = ThreadOrNewThread::Thread(Loadable::Failed(None));
                self.select_thread_in_list();
            },
            Msg::SetCreateThread => {
                if let Some(forum_id) = self.route.get_forum_id() {
                    let route = ForumRoute::CreateThread {forum_id};
                    context.routing.set_route(Route::Forums(route));
                }
            },
            Msg::SetThread {thread_id} => {
                if let Some(forum_id) = self.route.get_forum_id() {
                    let route = ForumRoute::Thread { forum_id, thread_id};
                    context.routing.set_route(Route::Forums(route.clone()));
                }
                self.thread = ThreadOrNewThread::Thread(Self::get_thread(thread_id, context));
            },
            Msg::SetForum {forum_data} => {
                let route = ForumRoute::Forum { forum_id: forum_data.id.clone() };
                context.routing.set_route(Route::Forums(route));
            }
            Msg::PostNewThread{new_thread} => {
                if let Some(forum_id) = self.route.get_forum_id() {
                    self.thread = ThreadOrNewThread::NewThread(Self::upload_new_thread(new_thread, forum_id, context))
                }
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        context.log(&format!("Forum: change() called: {:?}", props.route));


        use self::ForumRoute::*;
        match props.route {
            ForumList => {
                self.forums_or_selected_forum = ForumsOrForum::Forums(Self::get_forum_list(context));
                self.thread = ThreadOrNewThread::Thread(Loadable::Unloaded);
            },
            Forum {forum_id} => {
                let refresh: bool = if let ForumsOrForum::Forum{forum: Loadable::Loaded(ref forum_data), ..} = self.forums_or_selected_forum {
                    if forum_id != forum_data.id {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };

                if refresh {
                    self.forums_or_selected_forum = ForumsOrForum::Forum{forum: Self::get_forum(forum_id, context), threads: Self::get_threads(forum_id, context) }
                }
                self.thread = ThreadOrNewThread::Thread(Loadable::Unloaded);

            },
            Thread {thread_id, forum_id} => {
                let refresh_forum: bool = if let ForumsOrForum::Forum{forum: Loadable::Loaded(ref forum_data), ..} = self.forums_or_selected_forum {
                    if forum_id != forum_data.id {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };
                let refresh_thread: bool = if let ThreadOrNewThread::Thread(Loadable::Loaded(ref thread_data)) = self.thread {
                    if thread_id != thread_data.id {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };

                if refresh_forum {
                    self.forums_or_selected_forum = ForumsOrForum::Forum{forum: Self::get_forum(forum_id, context), threads: Self::get_threads(forum_id, context) };
                    self.thread = ThreadOrNewThread::Thread(Self::get_thread(thread_id, context))
                } else if refresh_thread {
                    self.thread = ThreadOrNewThread::Thread(Self::get_thread(thread_id, context))
                }
            },
            CreateThread {forum_id} => {
                let refresh: bool = if let ForumsOrForum::Forum{forum: Loadable::Loaded(ref forum_data), ..} = self.forums_or_selected_forum {
                    if forum_id != forum_data.id {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };

                if refresh {
                    self.forums_or_selected_forum = ForumsOrForum::Forum{forum: Self::get_forum(forum_id, context), threads: Self::get_threads(forum_id, context) }
                }
                self.thread = ThreadOrNewThread::NewThread(Default::default());
            }
        };
        true;

        self.route = props.route; // Set this here, in case it was forgotten earlier.
        true
    }
}
impl Renderable<Context, ForumModel> for ForumModel {
    fn view(&self) -> Html<Context, ForumModel> {


        fn forum_list_fn(forums: &Vec<ForumData>) -> Html<Context, ForumModel> {
            html! {
                <ul class=("forum-list"),>
                    { for forums.iter().map(ForumData::view) }
                </ul>
            }
        };


        fn thread_list_fn(threads: &Vec<SelectableMinimalThreadData>) -> Html<Context, ForumModel> {
            html! {
                <ul class=("forum-list"),>
                    { for threads.iter().map(SelectableMinimalThreadData::view) }
                </ul>
            }
        };

        fn thread_fn(thread: &ThreadData) -> Html<Context, ForumModel> {
            html! {
                <div>
                    <PostTree: post=&thread.posts, thread_id=thread.id, />
                </div>
            }
        }
        fn new_thread_fn(new_thread: &NewThreadData) -> Html<Context, ForumModel> {
            html! {
                <>
                    <NewThread: new_thread=new_thread, callback=|nt| Msg::PostNewThread{new_thread: nt}, />
                </>
            }
        }

        fn forum_title(forum_data: &ForumData) -> Html<Context,ForumModel> {
            html! {
                <div class=("flexbox-horiz"),>
                    {&forum_data.title}
                    <Button: title="Create New Thread", onclick=|_| Msg::SetCreateThread, />
                </div>
            }
        }
        fn forums_title(_: &Vec<ForumData>) -> Html<Context, ForumModel> {
            html! {
                <div> // TODO possibly switch to fragment and include horizontal boi div in area around callsite.
                    {"Forums"}
                </div>
            }
        }


        html!{
            <div class=("flexbox-vert","full-height", "no-scroll"),>
                <div class="forum-title",>
                    {
                        match self.forums_or_selected_forum {
                            ForumsOrForum::Forums(ref forums) => forums.default_view(forums_title),
                            ForumsOrForum::Forum{ref forum, ..} =>  forum.default_view(forum_title)
                        }
                    }
                </div>
                <div class=("flexbox-horiz", "full-height", "no-scroll"), > // Horizontal container
                    <div class=("vertical-expand", "list-background", "forum-list-width", "scrollable"),> // Vertical - list container
                       {
                            match self.forums_or_selected_forum {
                                ForumsOrForum::Forums(ref forums) => forums.default_view(forum_list_fn),
                                ForumsOrForum::Forum{ref threads ,..} =>  threads.default_view(thread_list_fn)
                            }
                        }
                    </div>
                    <div class=("vertical-expand", "full-width", "scrollable" ),> // Vertical - content container
                        {
                            match self.thread {
                                ThreadOrNewThread::Thread(ref thread) => thread.default_view(thread_fn),
                                ThreadOrNewThread::NewThread(ref new_thread) => new_thread.default_view(new_thread_fn)
                            }
                        }
                    </div>
                </div>
            </div>
        }

    }
}


