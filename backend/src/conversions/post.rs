use db::post::*;
use wire::post::*;
use chrono::Utc;
use db::thread::Thread;
use identifiers::post::PostUuid;


impl From<NewPostRequest> for NewPost {
    fn from(request: NewPostRequest) -> NewPost {
        NewPost {
            thread_uuid: request.thread_uuid.0,
            author_uuid: request.author_uuid.0,
            parent_uuid: request.parent_uuid.map(|x| x.0),
            created_date: Utc::now().naive_utc(),
            content: request.content,
            censored: false,
        }
    }
}

impl From<(Thread, String)> for NewPost {
    fn from(content: (Thread, String)) -> NewPost {
        NewPost {
            thread_uuid: content.0.uuid,
            author_uuid: content.0.author_uuid,
            parent_uuid: None,
            created_date: Utc::now().naive_utc(),
            content: content.1,
            censored: false,
        }
    }
}


impl From<EditPostRequest> for EditPostChangeset {
    fn from(request: EditPostRequest) -> EditPostChangeset {
        EditPostChangeset {
            uuid: request.uuid.0,
            modified_date: Utc::now().naive_utc(),
            content: request.content,
        }
    }
}



impl From<ChildlessPostData> for PostResponse {
    fn from(data: ChildlessPostData) -> PostResponse {
        PostResponse {
            uuid: PostUuid(data.post.uuid),
            author: data.user.into(),
            created_date: data.post.created_date,
            modified_date: data.post.modified_date,
            content: data.post.content,
            censored: data.post.censored,
            children: vec![],
        }
    }
}



impl From<PostData> for PostResponse {
    fn from(data: PostData) -> PostResponse {
        PostResponse {
            uuid: PostUuid(data.post.uuid),
            author: data.user.into(),
            created_date: data.post.created_date,
            modified_date: data.post.modified_date,
            content: data.post.content,
            censored: data.post.censored,
            children: data.children
                .into_iter()
                .map(PostResponse::from)
                .collect(),
        }
    }
}

impl From<ChildlessPostData> for PostData {
    fn from(childless: ChildlessPostData) -> PostData {
        PostData {
            post: childless.post,
            user: childless.user,
            children: vec![],
        }
    }
}
