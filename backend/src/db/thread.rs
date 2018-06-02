use schema::threads;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Forum;
use db::Conn;
use std::ops::Deref;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use error::JoeResult;

use db::post::{Post, NewPost};
use db::post::{PostData, ChildlessPostData};

#[derive(Debug, Clone, Identifiable, Associations, Queryable, Crd, ErrorHandler)]
#[insertable = "NewThread"]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Forum, foreign_key = "forum_id")]
#[table_name = "threads"]
pub struct Thread {
    /// Primary Key
    pub id: i32,
    /// Foreign Key to which the thread belongs to.
    pub forum_id: i32,
    /// Foreign Kay of the user who created the thread.
    pub author_id: i32,
    /// Timestamp of when the thread was created.
    pub created_date: NaiveDateTime,
    /// If the thread is locked, then it cannot be edited, nor can any of its posts.
    pub locked: bool,
    /// If the thread is archived, then it cannot be seen by non-moderators
    pub archived: bool,
    /// The title of the thread will be shown on think in the forum that will take you to the thread, as well as at the top of the thread's page.
    pub title: String,
}


#[derive(Insertable, Debug, Clone)]
#[table_name = "threads"]
pub struct NewThread {
    pub forum_id: i32,
    pub author_id: i32,
    pub created_date: NaiveDateTime,
    pub locked: bool,
    pub archived: bool,
    pub title: String,
}

pub struct ThreadData {
    pub thread: Thread,
    pub post: PostData,
    pub user: User,
}

pub struct MinimalThreadData {
    pub thread: Thread,
    pub user: User,
}

impl Thread {
    /// Locks or unlocks the thread, preventing posting and editing if locked
    pub fn set_lock_status(thread_id: i32, is_locked: bool, conn: &Conn) -> JoeResult<MinimalThreadData> {
        use schema::threads;
        use schema::threads::dsl::*;
        let thread: Thread = diesel::update(threads::table)
            .filter(id.eq(thread_id))
            .set(locked.eq(is_locked))
            .get_result(conn.deref())
            .map_err(Thread::handle_error)?;
        let user: User = User::get_by_id(thread.author_id, conn)?;

        Ok(MinimalThreadData { thread, user })
    }

    /// Archives the thread, preventing it from being seen in typical requests.
    pub fn archive_thread(thread_id: i32, conn: &Conn) -> JoeResult<MinimalThreadData> {
        use schema::threads;
        use schema::threads::dsl::*;
        let thread: Thread = diesel::update(threads::table)
            .filter(id.eq(thread_id))
            .set(archived.eq(true))
            .get_result(conn.deref())
            .map_err(Thread::handle_error)?;
        let user: User = User::get_by_id(thread.author_id, conn)?;

        Ok(MinimalThreadData { thread, user })
    }

    /// Gets all of the most recent threads in a forum.
    /// Archived threads will not be included.
    // TODO add a step to enable pagination
    pub fn get_threads_in_forum(requested_forum_id: i32, num_threads: i64, conn: &Conn) -> JoeResult<Vec<MinimalThreadData>> {
        use schema::threads::dsl::*;
        use db::forum::Forum;
        use schema::users::dsl::*;

        let forum: Forum = Forum::get_by_id(requested_forum_id, conn)?;

        // Get the threads that belong to the forum, and then get the users that are associated with the threads.
        let threads_and_users: Vec<(Thread, User)> = Thread::belonging_to(&forum)
            .filter(archived.eq(false))
            .order(created_date)
            .limit(num_threads)
            .inner_join(users)
            .load::<(Thread, User)>(conn.deref())
            .map_err(Thread::handle_error)?;


        let min_threads = threads_and_users
            .into_iter()
            .map(|x| {
                MinimalThreadData {
                    thread: x.0,
                    user: x.1,
                }
            })
            .collect();
        Ok(min_threads)
    }

    /// Gets threads based on page size and index.
    pub fn get_paginated(requested_forum_id: i32, page_index: i32, page_size: i32, conn: &Conn) -> JoeResult<Vec<MinimalThreadData>> {
        use schema::threads::dsl::*;
        use db::forum::Forum;
        use db::diesel_extensions::pagination::*;
        use schema::users;

        let forum: Forum = Forum::get_by_id(requested_forum_id, conn)?;

        let (thread_users, _count) = Thread::belonging_to(&forum)
            .inner_join(users::table)
            .order(created_date)
            .filter(archived.eq(false))
            .paginate(page_index.into())
            .per_page(page_size.into())
            .load_and_count_pages::<(Thread, User)>(conn.deref())
            .map_err(Thread::handle_error)?;

        let minimal_threads = thread_users
            .into_iter()
            .map(|x| {
                MinimalThreadData {
                    thread: x.0,
                    user: x.1,
                }
            })
            .collect();

        Ok(minimal_threads)


    }


    /// Creates a thread with an initial post.
    pub fn create_thread_with_initial_post(new_thread: NewThread, post_content: String, conn: &Conn) -> JoeResult<ThreadData> {
        let thread: Thread = Thread::create(new_thread, conn)?;

        let new_post: NewPost = NewPost::from((thread.clone(), post_content));

        let post_data: ChildlessPostData = Post::create_and_get_user(new_post, conn)?;
        let user: User = post_data.user.clone();
        Ok(ThreadData {
            thread,
            post: PostData::from(post_data),
            user,
        })
    }

    /// Gets every bit of data related to a thread.
    pub fn get_full_thread(thread_id: i32, conn: &Conn) -> JoeResult<ThreadData> {
        let thread: Thread = Thread::get_by_id(thread_id, conn)?;
        let root_post: Post = Post::get_root_post(thread_id, conn)?;
        let post: PostData = root_post.get_post_data(conn)?;
        let user = User::get_by_id(thread.author_id, conn)?;
        Ok(ThreadData { thread, post, user })
    }
}
