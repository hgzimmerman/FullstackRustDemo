use chrono::Utc;
use diesel::PgConnection;
use Fixture;

use db::{
    forum::{
        Forum,
        NewForum,
    },
    post::{
        NewPost,
        Post,
    },
    thread::{
        NewThread,
        Thread,
    },
};

use fixtures::user::UserFixture;

const FORUM_TITLE: &'static str = "Forum Title";
const FORUM_DESCRIPTION: &'static str = "Forum Description";

const THREAD_1_TITLE: &'static str = "Thread 1 Title";
const THREAD_2_TITLE: &'static str = "Thread 2 Title";

const POST_1_CONTENT: &'static str = "Post 1 content";
const POST_2_CONTENT: &'static str = "Post 2 content";
const POST_3_CONTENT: &'static str = "Post 3 content";

pub struct ForumFixture {
    pub user_fixture: UserFixture,
    pub forum: Forum,
    pub populated_thread: Thread,
    pub empty_thread: Thread,
    pub post_1: Post,
    pub post_2: Post,
    pub post_3: Post,
}

impl Fixture for ForumFixture {
    fn generate(conn: &PgConnection) -> Self {
        let user_fixture: UserFixture = UserFixture::generate(conn);

        let new_forum: NewForum = NewForum {
            title: String::from(FORUM_TITLE),
            description: String::from(FORUM_DESCRIPTION),
        };
        let forum = Forum::create_forum(new_forum, conn).expect("create forum");

        let new_thread_1: NewThread = NewThread {
            forum_uuid: forum.uuid,
            author_uuid: user_fixture.normal_user.uuid,
            created_date: Utc::now().naive_utc(),
            locked: false,
            archived: false,
            title: THREAD_1_TITLE.to_string(),
        };
        let mut new_thread_2: NewThread = new_thread_1.clone();
        new_thread_2.title = THREAD_2_TITLE.to_string();

        let thread_1 = Thread::create_thread(new_thread_1, conn).expect("create thread");
        let thread_2 = Thread::create_thread(new_thread_2, conn).expect("create thread");

        let new_post_1: NewPost = NewPost {
            thread_uuid: thread_1.uuid,
            author_uuid: user_fixture.normal_user.uuid,
            parent_uuid: None,
            created_date: Utc::now().naive_utc(),
            content: POST_1_CONTENT.to_string(),
            censored: false,
        };

        let mut new_post_2: NewPost = new_post_1.clone();
        let mut new_post_3: NewPost = new_post_1.clone();

        let post_1: Post = Post::create_post(new_post_1, conn).expect("create post");

        new_post_2.content = POST_2_CONTENT.to_string();
        new_post_3.content = POST_3_CONTENT.to_string();
        new_post_2.parent_uuid = Some(post_1.uuid);
        new_post_3.parent_uuid = Some(post_1.uuid);

        let post_2: Post = Post::create_post(new_post_2, conn).expect("create post");
        let post_3: Post = Post::create_post(new_post_3, conn).expect("create post");

        ForumFixture {
            user_fixture,
            forum,
            populated_thread: thread_1,
            empty_thread: thread_2,
            post_1,
            post_2,
            post_3,
        }
    }
}
