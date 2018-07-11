use common::fetch::Auth;
use common::fetch::FetchRequest;
use common::fetch::HttpMethod;
use common::fetch::to_body;
use identifiers::forum::ForumUuid;
use identifiers::thread::ThreadUuid;
use wire::post::*;
use wire::thread::*;
use wire::forum::NewForumRequest;

#[derive(Serialize, Deserialize)]
pub enum ForumRequest {
    CreateThread(NewThreadRequest),
    CreateForum(NewForumRequest),
    GetThreads { forum_uuid: ForumUuid, page_index: usize },
    GetForums,
    GetForum { forum_uuid: ForumUuid },
    GetThread { thread_uuid: ThreadUuid },
    CreatePostResponse(NewPostRequest),
    UpdatePost(EditPostRequest),
}

impl FetchRequest for ForumRequest {
    fn resolve_path(&self) -> String {
        use self::ForumRequest::*;
        match *self {
            CreateThread(_) => "thread/create".into(),
            CreateForum(_) => "forum/create".into(),
            GetThreads {
                forum_uuid,
                page_index,
            } => format!("thread/get/{}/{}", forum_uuid, page_index),
            GetForums => "forum/forums".into(),
            GetForum { forum_uuid } => format!("forum/{}", forum_uuid),
            GetThread { thread_uuid } => format!("thread/{}", thread_uuid),
            CreatePostResponse(_) => "post/create".into(),
            UpdatePost(_) => "post/edit".into(),
        }
    }
    fn resolve_auth(&self) -> Auth {
        use self::ForumRequest::*;
        use self::Auth::*;
        match *self {
            CreateThread(_) => Required,
            CreateForum(_) => Required, // Admin only
            GetThreads {..} =>  NotRequired,
            GetForums => NotRequired,
            GetForum {..} => NotRequired,
            GetThread {..} => NotRequired,
            CreatePostResponse(_) => Required,
            UpdatePost(_) => Required,
        }

    }
    fn resolve_body_and_method(&self) -> HttpMethod {
        use self::ForumRequest::*;
        use self::HttpMethod::*;
        match self {
            CreateThread(r) => Post(to_body(r)),
            CreateForum(r) => Post(to_body(r)),
            GetThreads {..} => Get,
            GetForums => Get,
            GetForum {..} => Get,
            GetThread {..} => Get,
            CreatePostResponse(r) => Post(to_body(r)),
            UpdatePost(r) => Put(to_body(r)),
        }
    }
}