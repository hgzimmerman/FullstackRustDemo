use db::thread::{Thread, MinimalThreadData};
use db::post::{Post, NewPost, EditPostChangeset, PostData, PostVote, Vote, VoteCounts};
use common::setup::*;
use diesel::PgConnection;
use chrono::Utc;
use identifiers::forum::ForumUuid;
use identifiers::thread::ThreadUuid;
use identifiers::post::PostUuid;
use uuid::Uuid;
use test::Bencher;
use testing_fixtures::fixtures::forum::ForumFixture;
use identifiers::user::UserUuid;



#[test]
fn get_paginated() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let forum_uuid = ForumUuid(fixture.forum.uuid);
        let threads: Vec<MinimalThreadData> = Thread::get_paginated(forum_uuid, 1, 10, conn)
            .expect("get threads in forum");

        let thread_uuids: Vec<Uuid> = threads.into_iter().map(|x| x.thread.uuid).collect();

        assert!(thread_uuids.contains(&fixture.populated_thread.uuid));
        assert!(thread_uuids.contains(&fixture.empty_thread.uuid));


        // Won't get anything in the second index
        let threads: Vec<MinimalThreadData> = Thread::get_paginated(forum_uuid, 2, 10, conn)
            .expect("get threads in forum");

        assert_eq!(threads.len(), 0);
    })
}

/// After archiving a thread, it should not be possible to get it with the get_threads_in_forum method,
/// and it should not be possible to modify its child posts.
///
/// Under typical circumstances, the thread should not be accessable.
#[test]
fn archive() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let forum_uuid = ForumUuid(fixture.forum.uuid);
        let thread_1_uuid = ThreadUuid(fixture.populated_thread.uuid);

        // get post to respond to.
        let root_post: Post = Post::get_root_post(thread_1_uuid, conn)
            .expect("get root post");

        Thread::archive_thread(thread_1_uuid, conn).expect("Archive thread");

        let threads: Vec<MinimalThreadData> = Thread::get_paginated(forum_uuid, 1, 10, conn)
            .expect("get threads in forum");

        let thread_uuids: Vec<Uuid> = threads.into_iter().map(|x| x.thread.uuid).collect();

        assert!(!thread_uuids.contains(&fixture.populated_thread.uuid));
        assert!(thread_uuids.contains(&fixture.empty_thread.uuid));


        Post::get_root_post(thread_1_uuid, conn)
            .expect_err("should not be able to get root post of locked thread");


        // Should still be accepted
        let new_post: NewPost = NewPost {
            thread_uuid: fixture.populated_thread.uuid,
            author_uuid: fixture.user_fixture.normal_user.uuid,
            parent_uuid: Some(root_post.uuid),
            created_date: Utc::now().naive_utc(),
            content: "New content".to_string(),
            censored: false,
        };
        Post::create_and_get_user(new_post, conn)
            .expect_err("Should not be able to create a new post if the thread is archived.");
    })
}

#[test]
fn lock() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let thread_1_uuid = ThreadUuid(fixture.populated_thread.uuid);

        // get post to respond to.
        let root_post: Post = Post::get_root_post(thread_1_uuid, conn)
            .expect("get root post");

        Thread::set_lock_status(thread_1_uuid, true,conn).expect("lock thread");


        // Should not be accepted
        let new_post: NewPost = NewPost {
            thread_uuid: fixture.populated_thread.uuid,
            author_uuid: fixture.user_fixture.normal_user.uuid,
            parent_uuid: Some(root_post.uuid),
            created_date: Utc::now().naive_utc(),
            content: "New Content".to_string(),
            censored: false,
        };
        Post::create_and_get_user(new_post, conn)
            .expect_err("Should not be able to create a new post if the thread is locked");

        let changeset = EditPostChangeset {
            uuid: root_post.uuid,
            modified_date: Utc::now().naive_utc(),
            content: String::from("Changed content"),
        };

        let user_uuid = UserUuid(fixture.user_fixture.normal_user.uuid);

        Post::modify_post(changeset, thread_1_uuid, user_uuid,conn)
            .expect_err("Should not be able to modify post");
    })
}

#[test]
fn get_post_and_children() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        // post one has two children
        let post_uuid = PostUuid(fixture.post_1.uuid);
        let post_data: PostData = Post::get_post_and_children(post_uuid, None,conn).expect("should get post and its children");
        assert_eq!(post_data.children.len(), 2);

        // Post two has no children
        let post_uuid = PostUuid(fixture.post_2.uuid);
        let post_data: PostData = Post::get_post_and_children(post_uuid, None,conn).expect("should get post and its children");
        assert_eq!(post_data.children.len(), 0);

    });
}

#[test]
fn voting_simple_vote() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let post_uuid = PostUuid(fixture.post_1.uuid);
        let user_uuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let vote = Vote {
            post_uuid,
            user_uuid
        };
        let upvote = PostVote::Up(vote);

        Post::vote(upvote, conn).expect("Vote should be cast");

        let post = Post::get_individual_post(post_uuid, user_uuid, conn).expect("Should get post");
        let vote_counts: VoteCounts = post.votes;
        assert_eq!(vote_counts.up, 1);
        assert_eq!(vote_counts.down, 0);
        assert!(vote_counts.user_voted_up);
    });
}

#[test]
fn voting_up_then_down() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let post_uuid = PostUuid(fixture.post_1.uuid);
        let user_uuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let vote = Vote {
            post_uuid,
            user_uuid
        };
        let upvote = PostVote::Up(vote);

        Post::vote(upvote, conn).expect("Vote should be cast");

        let post = Post::get_individual_post(post_uuid, user_uuid, conn).expect("Should get post");
        let vote_counts: VoteCounts = post.votes;
        assert_eq!(vote_counts.up, 1);
        assert_eq!(vote_counts.down, 0);
        assert!(vote_counts.user_voted_up);

        let downvote = PostVote::Down(vote);
        Post::vote(downvote, conn).expect("Vote should be cast");
        let post = Post::get_individual_post(post_uuid, user_uuid, conn).expect("Should get post");
        let vote_counts: VoteCounts = post.votes;
        assert_eq!(vote_counts.up, 0);
        assert_eq!(vote_counts.down, 1);
        assert!(vote_counts.user_voted_down);
        assert!(!vote_counts.user_voted_up);
    });
}

/// Voting twice as one user should not increment the up vote.
#[test]
fn voting_twice() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let post_uuid = PostUuid(fixture.post_1.uuid);
        let user_uuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let vote = Vote {
            post_uuid,
            user_uuid
        };
        let upvote = PostVote::Up(vote);

        Post::vote(upvote, conn).expect("Vote should be cast");

        let post = Post::get_individual_post(post_uuid, user_uuid, conn).expect("Should get post");
        let vote_counts: VoteCounts = post.votes;
        assert_eq!(vote_counts.up, 1);
        assert_eq!(vote_counts.down, 0);
        assert!(vote_counts.user_voted_up);

        let upvote = PostVote::Up(vote);
        Post::vote(upvote, conn).expect_err("Should not be able to vote twice");
        let post = Post::get_individual_post(post_uuid, user_uuid, conn).expect("Should get post");
        let vote_counts: VoteCounts = post.votes;
        assert_eq!(vote_counts.up, 1);
    });
}
#[test]
fn voting_revocation() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let post_uuid = PostUuid(fixture.post_1.uuid);
        let user_uuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let vote = Vote {
            post_uuid,
            user_uuid
        };
        let upvote = PostVote::Up(vote);
        Post::vote(upvote, conn).expect("Vote should be cast");

        Post::revoke_vote(user_uuid, post_uuid, conn).expect("vote should be revoked");

        let downvote = PostVote::Down(vote);
        Post::vote(downvote, conn).expect("Vote should be cast");

        Post::revoke_vote(user_uuid, post_uuid, conn).expect("vote should be revoked");
    });
}

#[test]
fn voting_many() {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let post_1_uuid = PostUuid(fixture.post_1.uuid);
        let post_2_uuid = PostUuid(fixture.post_2.uuid);
        let normal_user_uuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let admin_user_uuid = UserUuid(fixture.user_fixture.admin_user.uuid);

        let vote = Vote {
            post_uuid: post_1_uuid,
            user_uuid: normal_user_uuid
        };
        let upvote = PostVote::Up(vote);
        Post::vote(upvote, conn).expect("Vote should be cast");

        let vote = Vote {
            post_uuid: post_1_uuid,
            user_uuid: admin_user_uuid
        };
        let upvote = PostVote::Up(vote);
        Post::vote(upvote, conn).expect("Vote should be cast");

        let vote = Vote {
            post_uuid: post_2_uuid,
            user_uuid: normal_user_uuid
        };
        let upvote = PostVote::Up(vote);
        Post::vote(upvote, conn).expect("Vote should be cast");

        let posts: Vec<Post> = vec![fixture.post_1.clone(), fixture.post_2.clone(), fixture.post_3.clone()];
        let posts_and_votes = Post::get_votes_for_posts(&posts, Some(admin_user_uuid), conn).expect("should get vote counts");

        assert_eq!(posts_and_votes.len(), 3);
        assert_eq!(posts_and_votes[0].up, 2);
        assert!(posts_and_votes[0].user_voted_up);
        assert_eq!(posts_and_votes[1].up, 1);
        assert!(!posts_and_votes[1].user_voted_up);

    });
}


#[bench]
fn get_posts(b: &mut Bencher) {
    setup(|fixture: &ForumFixture, conn: &PgConnection| {
        let mut new_post: NewPost = NewPost {
            thread_uuid: fixture.empty_thread.uuid, // should be thread without posts
            author_uuid: fixture.user_fixture.normal_user.uuid,
            parent_uuid: None,
            created_date: Utc::now().naive_utc(),
            content: "New Content".to_string(),
            censored: false,
        };
        // create 100 posts all in a row.
        for _ in 0..100 {
            let post_uuid: Uuid = Post::create_and_get_user(new_post.clone(), conn)
                .map(|x| x.post.uuid)
                .expect("Create post");
            new_post.parent_uuid = Some(post_uuid);
        }

        let thread_2_uuid = ThreadUuid(fixture.empty_thread.uuid);

        b.iter(
            || {
                Post::get_posts_in_thread(thread_2_uuid, None, conn).expect("Should get post tree")
            },
        );

    });
}
