use calls::user::UserFixture;
use db::forum::{Forum, NewForum};
use db::thread::{Thread, NewThread, MinimalThreadData};
use db::post::{Post, NewPost, EditPostChangeset, PostData};
use common::setup::*;
use diesel::PgConnection;
use chrono::Utc;
use db::CreatableUuid;
use identifiers::forum::ForumUuid;
use identifiers::thread::ThreadUuid;
use identifiers::post::PostUuid;
use uuid::Uuid;
use test::Bencher;


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
    pub thread_1: Thread,
    pub thread_2: Thread,
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
        let forum = Forum::create(new_forum, conn).expect("create forum");


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

        let thread_1 = Thread::create(new_thread_1, conn).expect("create thread");
        let thread_2 = Thread::create(new_thread_2, conn).expect("create thread");


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

        let post_1: Post = Post::create(new_post_1, conn).expect("create post");

        new_post_2.content = POST_2_CONTENT.to_string();
        new_post_3.content = POST_3_CONTENT.to_string();
        new_post_2.parent_uuid = Some(post_1.uuid);
        new_post_3.parent_uuid = Some(post_1.uuid);


        let post_2: Post = Post::create(new_post_2, conn).expect("create post");
        let post_3: Post = Post::create(new_post_3, conn).expect("create post");

        ForumFixture {
            user_fixture,
            forum,
            thread_1,
            thread_2,
            post_1,
            post_2,
            post_3,
        }
    }
}

#[test]
fn get_paginated() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let forum_uuid = ForumUuid(fixture.forum.uuid);
        let threads: Vec<MinimalThreadData> = Thread::get_paginated(forum_uuid, 1, 10, conn)
            .expect("get threads in forum");

        let thread_uuids: Vec<Uuid> = threads.into_iter().map(|x| x.thread.uuid).collect();

        assert!(thread_uuids.contains(&fixture.thread_1.uuid));
        assert!(thread_uuids.contains(&fixture.thread_2.uuid));


        // Won't get anything in the second index
        let threads: Vec<MinimalThreadData> = Thread::get_paginated(forum_uuid, 2, 10, conn)
            .expect("get threads in forum");

        assert_eq!(threads.len(), 0);
    })
}

/// After archiving a thread, it should not be possible to get it with the get_threads_in_forum method,
/// and it should not be possible to modify its child posts.
#[test]
fn archive() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let forum_uuid = ForumUuid(fixture.forum.uuid);
        let thread_1_uuid = ThreadUuid(fixture.thread_1.uuid);

        // get post to respond to.
        let root_post: Post = Post::get_root_post(thread_1_uuid, conn)
            .expect("get root post");

        Thread::archive_thread(thread_1_uuid, conn).expect("Archive thread");

        let threads: Vec<MinimalThreadData> = Thread::get_paginated(forum_uuid, 1, 10, conn)
            .expect("get threads in forum");

        let thread_uuids: Vec<Uuid> = threads.into_iter().map(|x| x.thread.uuid).collect();

        assert!(!thread_uuids.contains(&fixture.thread_1.uuid));
        assert!(thread_uuids.contains(&fixture.thread_2.uuid));


        Post::get_root_post(thread_1_uuid, conn)
            .expect_err("should not be able to get root post of locked thread");


        // Should still be accepted
        let new_post: NewPost = NewPost {
            thread_uuid: fixture.thread_1.uuid,
            author_uuid: fixture.user_fixture.normal_user.uuid,
            parent_uuid: Some(root_post.uuid),
            created_date: Utc::now().naive_utc(),
            content: POST_1_CONTENT.to_string(),
            censored: false,
        };
        Post::create_and_get_user(new_post, conn)
            .expect_err("Should not be able to create a new post if the thread is archived.");
    })
}

#[test]
fn lock() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let thread_1_uuid = ThreadUuid(fixture.thread_1.uuid);

        // get post to respond to.
        let root_post: Post = Post::get_root_post(thread_1_uuid, conn)
            .expect("get root post");

        Thread::set_lock_status(thread_1_uuid, true,conn).expect("lock thread");


        // Should not be accepted
        let new_post: NewPost = NewPost {
            thread_uuid: fixture.thread_1.uuid,
            author_uuid: fixture.user_fixture.normal_user.uuid,
            parent_uuid: Some(root_post.uuid),
            created_date: Utc::now().naive_utc(),
            content: POST_1_CONTENT.to_string(),
            censored: false,
        };
        Post::create_and_get_user(new_post, conn)
            .expect_err("Should not be able to create a new post if the thread is locked");

        let changeset = EditPostChangeset {
            uuid: root_post.uuid,
            modified_date: Utc::now().naive_utc(),
            content: String::from("Changed content"),
        };

        Post::modify_post(changeset, thread_1_uuid, conn)
            .expect_err("Should not be able to modify post");
    })
}

#[test]
fn get_post_and_children() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        // post one has two children
        let post_uuid = PostUuid(fixture.post_1.uuid);
        let post_data: PostData = Post::get_post_and_children(post_uuid, conn).expect("should get post and its children");
        assert_eq!(post_data.children.len(), 2);

        // Post two has no children
        let post_uuid = PostUuid(fixture.post_2.uuid);
        let post_data: PostData = Post::get_post_and_children(post_uuid, conn).expect("should get post and its children");
        assert_eq!(post_data.children.len(), 0);


    });
}

#[bench]
fn get_posts(b: &mut Bencher) {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let mut new_post: NewPost = NewPost {
            thread_uuid: fixture.thread_2.uuid, // should be empty thread
            author_uuid: fixture.user_fixture.normal_user.uuid,
            parent_uuid: None,
            created_date: Utc::now().naive_utc(),
            content: POST_1_CONTENT.to_string(),
            censored: false,
        };
        // create 100 posts all in a row.
        for _ in 0..100 {
            let post_uuid: Uuid = Post::create_and_get_user(new_post.clone(), conn)
                .map(|x| x.post.uuid)
                .expect("Create post");
            new_post.parent_uuid = Some(post_uuid);
        }

        let thread_2_uuid = ThreadUuid(fixture.thread_2.uuid);

        b.iter(
            || {
                Post::get_posts_in_thread(thread_2_uuid, conn).expect("Should get post tree")
            },
        );

    });
}
