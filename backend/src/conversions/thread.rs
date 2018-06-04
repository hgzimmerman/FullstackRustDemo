use db::thread::*;
use wire::thread::*;
use chrono::Utc;
use identifiers::thread::ThreadUuid;
use identifiers::forum::ForumUuid;

impl From<NewThreadRequest> for NewThread {
    fn from(request: NewThreadRequest) -> NewThread {
        NewThread {
            forum_uuid: request.forum_uuid.0,
            author_uuid: request.author_uuid.0,
            created_date: Utc::now().naive_utc(),
            locked: false,
            archived: false,
            title: request.title,
        }
    }
}



impl From<ThreadData> for ThreadResponse {
    fn from(data: ThreadData) -> ThreadResponse {
        ThreadResponse {
            uuid: ThreadUuid(data.thread.uuid),
            forum_uuid: ForumUuid(data.thread.forum_uuid),
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
            uuid: ThreadUuid(data.thread.uuid),
            title: data.thread.title,
            author: data.user.into(),
            created_date: data.thread.created_date,
            locked: data.thread.locked,
        }
    }
}
