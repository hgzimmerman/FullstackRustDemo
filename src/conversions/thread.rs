use db::thread::*;
use requests_and_responses::thread::*;
use db::post::NewPost;
use chrono::Utc;

impl From<NewThreadRequest> for NewThread {
    fn from(request: NewThreadRequest) -> NewThread {
        NewThread {
            forum_id: request.forum_id,
            author_id: request.author_id,
            created_date: Utc::now().naive_utc(),
            locked: false,
            archived: false,
            title: request.title,
        }
    }
}

impl From<NewThreadRequest> for NewPost {
    fn from(request: NewThreadRequest) -> NewPost {
        // Just grab the post field from the thread request.
        NewPost::from(request.post)
    }
}



impl From<ThreadData> for ThreadResponse {
    fn from(data: ThreadData) -> ThreadResponse {
        ThreadResponse {
            id: data.thread.id,
            title: data.thread.title,
            author: data.user.into(),
            posts: data.post.into(),
            created_date: data.thread.created_date,
            locked: data.thread.locked,
        }
    }
}



impl From<MinimalThreadData> for MinimalThreadResponse {
    fn from(data: MinimalThreadData) -> MinimalThreadResponse {
        MinimalThreadResponse {
            id: data.thread.id,
            title: data.thread.title,
            author: data.user.into(),
            created_date: data.thread.created_date,
            locked: data.thread.locked,
        }
    }
}
