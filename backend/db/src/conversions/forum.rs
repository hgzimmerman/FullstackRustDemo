use crate::forum::*;
use identifiers::forum::ForumUuid;
use wire::forum::*;

impl From<Forum> for ForumResponse {
    fn from(forum: Forum) -> ForumResponse {
        ForumResponse {
            uuid: ForumUuid(forum.uuid),
            title: forum.title,
            description: forum.description,
        }
    }
}

impl From<NewForumRequest> for NewForum {
    fn from(new_forum_request: NewForumRequest) -> NewForum {
        NewForum {
            title: new_forum_request.title,
            description: new_forum_request.description,
        }
    }
}
