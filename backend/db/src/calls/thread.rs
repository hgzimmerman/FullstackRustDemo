use crate::schema::threads;
use chrono::NaiveDateTime;
use crate::user::User;
use crate::forum::Forum;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use error::JoeResult;
use diesel::PgConnection;
use uuid::Uuid;
use identifiers::thread::ThreadUuid;
use identifiers::user::UserUuid;

use crate::post::{Post, NewPost};
use crate::post::{PostData, ChildlessPostData};
use identifiers::forum::ForumUuid;
use crate::calls::prelude::*;
use crate::schema;


#[derive(Debug, Clone, Identifiable, Associations, Queryable, TypeName)]
#[primary_key(uuid)]
//#[insertable = "NewThread"]
#[belongs_to(User, foreign_key = "author_uuid")]
#[belongs_to(Forum, foreign_key = "forum_uuid")]
#[table_name = "threads"]
pub struct Thread {
    /// Primary Key
    pub uuid: Uuid,
    /// Foreign Key to which the thread belongs to.
    pub forum_uuid: Uuid,
    /// Foreign Kay of the user who created the thread.
    pub author_uuid: Uuid,
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
    pub forum_uuid: Uuid,
    pub author_uuid: Uuid,
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

    pub fn get_thread(uuid: ThreadUuid,conn: &PgConnection) -> JoeResult<Thread> {
        get_row::<Thread,_>(schema::threads::table, uuid.0, conn)
    }
    pub fn delete_thread(uuid: ThreadUuid, conn: &PgConnection) -> JoeResult<Thread> {
        delete_row::<Thread,_>(schema::threads::table, uuid.0, conn)
    }
    pub fn create_thread(new: NewThread, conn: &PgConnection) -> JoeResult<Thread> {
        create_row::<Thread, NewThread,_>(schema::threads::table, new, conn)
    }


    /// Locks or unlocks the thread, preventing posting and editing if locked
    pub fn set_lock_status(thread_uuid: ThreadUuid, is_locked: bool, conn: &PgConnection) -> JoeResult<MinimalThreadData> {
        use crate::schema::threads;
        use crate::schema::threads::dsl::*;

        let thread: Thread = diesel::update(threads::table)
            .filter(threads::uuid.eq(thread_uuid.0))
            .set(locked.eq(is_locked))
            .get_result(conn)
            .map_err(handle_err::<Thread>)?;

        let author_uuid_a = UserUuid(thread.author_uuid);
        let user: User = User::get_user(author_uuid_a, conn)?;

        Ok(MinimalThreadData { thread, user })
    }

    /// Archives the thread, preventing it from being seen in typical requests.
    ///
    /// The thread _must_ also be locked in order to not be modifiable.
    pub fn archive_thread(thread_uuid: ThreadUuid, conn: &PgConnection) -> JoeResult<MinimalThreadData> {
        use crate::schema::threads;
        use crate::schema::threads::dsl::*;

        let m_thread_uuid: Uuid = thread_uuid.0;

        let thread: Thread = diesel::update(threads::table)
            .filter(threads::uuid.eq(m_thread_uuid))
            .set(archived.eq(true))
            .get_result(conn)
            .map_err(handle_err::<Thread>)?;
        let author_uuid_a = UserUuid(thread.author_uuid);
        let user: User = User::get_user(author_uuid_a, conn)?;

        Ok(MinimalThreadData { thread, user })
    }

    /// Gets all of the most recent threads in a forum.
    /// Archived threads will not be included.
    #[deprecated]
    pub fn get_threads_in_forum(requested_forum_uuid: ForumUuid, num_threads: i64, conn: &PgConnection) -> JoeResult<Vec<MinimalThreadData>> {
        use crate::schema::threads::dsl::*;
        use crate::forum::Forum;
        use crate::schema::users::dsl::*;

        let forum: Forum = Forum::get_forum(requested_forum_uuid, conn)?;

        // Get the threads that belong to the forum, and then get the users that are associated with the threads.
        let threads_and_users: Vec<(Thread, User)> = Thread::belonging_to(&forum)
            .filter(archived.eq(false))
            .order(created_date)
            .limit(num_threads)
            .inner_join(users)
            .load::<(Thread, User)>(conn)
            .map_err(handle_err::<Thread>)?;


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
    pub fn get_paginated(requested_forum_uuid: ForumUuid, page_index: i32, page_size: i32, conn: &PgConnection) -> JoeResult<Vec<MinimalThreadData>> {
        use crate::schema::threads::dsl::*;
        use crate::forum::Forum;
        use crate::diesel_extensions::pagination::*;
        use crate::schema::users;

        let forum: Forum = Forum::get_forum(requested_forum_uuid, conn)?;

        let (thread_users, _count) = Thread::belonging_to(&forum)
            .inner_join(users::table)
            .order(created_date)
            .filter(archived.eq(false))
            .paginate(page_index.into())
            .per_page(page_size.into())
            .load_and_count_pages::<(Thread, User)>(conn)
            .map_err(handle_err::<Thread>)?;

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
    pub fn create_thread_with_initial_post(new_thread: NewThread, post_content: String, conn: &PgConnection) -> JoeResult<ThreadData> {
        let thread: Thread = Thread::create_thread(new_thread, conn)?;

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
    pub fn get_full_thread(thread_uuid: ThreadUuid, user_uuid: Option<UserUuid>, conn: &PgConnection) -> JoeResult<ThreadData> {
        let thread: Thread = Thread::get_thread(thread_uuid, conn)?;
        let post: PostData = Post::get_posts_in_thread(thread_uuid, user_uuid, conn)?;
        let author_uuid = UserUuid(thread.author_uuid);
        let user = User::get_user(author_uuid, conn)?;
        Ok(ThreadData { thread, post, user })
    }
}
