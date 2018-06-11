use schema::posts;
use chrono::NaiveDateTime;
use user::User;
use thread::Thread;
use error::*;
use diesel;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::BelongingToDsl;
use diesel::QueryDsl;
use error::JoeResult;
use diesel::SaveChangesDsl;
use diesel::PgConnection;
use identifiers::post::PostUuid;
use identifiers::thread::ThreadUuid;
use identifiers::user::UserUuid;
use uuid::Uuid;
use chrono::Utc;
use log::info;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Identifiable, Associations, Queryable, CrdUuid, ErrorHandler)]
#[primary_key(uuid)]
#[insertable = "NewPost"]
#[belongs_to(User, foreign_key = "author_uuid")]
#[belongs_to(Thread, foreign_key = "thread_uuid")]
#[belongs_to(Post, foreign_key = "parent_uuid")]
#[table_name = "posts"]
pub struct Post {
    /// Primary Key
    pub uuid: Uuid,
    /// The Foreign Key of the thread the post belongs to.
    pub thread_uuid: Uuid,
    /// The Foreign Key of the user that created the post.
    pub author_uuid: Uuid,
    /// The Foreign Key of the post to which this post is replying to. None indicates that this is the OP for the thread.
    pub parent_uuid: Option<Uuid>,
    /// The timestamp of when the post was created.
    pub created_date: NaiveDateTime,
    /// If the post was edited, the most recent edit time will be attached to the post.
    pub modified_date: Option<NaiveDateTime>,
    /// The content of the post. This may be rendered with markdown or a subset thereof.
    pub content: String,
    /// If the post has been censored, it will not be immediately viewable by people viewing the thread.
    pub censored: bool,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "posts"]
pub struct NewPost {
    pub thread_uuid: Uuid,
    pub author_uuid: Uuid,
    pub parent_uuid: Option<Uuid>,
    pub created_date: NaiveDateTime,
    pub content: String,
    pub censored: bool,
}

#[derive(AsChangeset, Debug, Identifiable)]
#[primary_key(uuid)]
#[table_name = "posts"]
pub struct EditPostChangeset {
    pub uuid: Uuid,
    pub modified_date: NaiveDateTime,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostData {
    pub post: Post,
    pub user: User,
    pub children: Vec<PostData>,
}

#[derive(Debug, Clone)]
pub struct ChildlessPostData {
    pub post: Post,
    pub user: User,
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

impl From<ChildlessPostData> for PostData {
    fn from(childless: ChildlessPostData) -> PostData {
        PostData {
            post: childless.post,
            user: childless.user,
            children: vec![],
        }
    }
}

impl Post {
    /// Applies the EditPostChangeset to the post.
    /// If the thread is locked, the post cannot be modified
    pub fn modify_post(edit_post_changeset: EditPostChangeset, thread_uuid: ThreadUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        //        use schema::posts;

        let target_thread: Thread = Thread::get_by_uuid(thread_uuid.0, conn)?;
        if target_thread.locked || target_thread.archived {
            return Err(WeekendAtJoesError::ThreadImmutable);
        }


        let modified_post: Post = edit_post_changeset
            .save_changes(conn)
            .map_err(Post::handle_error)?;
        let user = User::get_by_uuid(modified_post.author_uuid, conn)?;
        Ok(ChildlessPostData {
            post: modified_post,
            user,
        })
    }


    /// Creates a post, and also gets the associated author for the post.
    pub fn create_and_get_user(new_post: NewPost, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        let thread: Thread = Thread::get_by_uuid(new_post.thread_uuid, conn)?;
        if thread.locked || thread.archived {
            return Err(WeekendAtJoesError::ThreadImmutable);
        }

        // Do not allow the post to be created if the thread already has an "original post"
        if let None = new_post.parent_uuid {
            let thread_uuid: ThreadUuid = ThreadUuid(thread.uuid);
            // Expect this to return
            if let Err(_) = Post::get_root_post(thread_uuid, conn) {
                info!("New post created for new thread.");
            } else {
                return Err(WeekendAtJoesError::BadRequest) // TODO need better error
            }
        };

        let post: Post = Post::create(new_post, conn)?;
        let user: User = User::get_by_uuid(post.author_uuid, conn)?;
        Ok(ChildlessPostData { post, user })
    }

    /// Censors the post, preventing users from seeing it by default.
    pub fn censor_post(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        use schema::posts::dsl::*;
        use schema::posts;

        let m_post_uuid: Uuid = post_uuid.0;

        let censored_post: Post = diesel::update(posts::table)
            .filter(posts::uuid.eq(m_post_uuid))
            .set(censored.eq(true))
            .get_result(conn)
            .map_err(Post::handle_error)?;
        let user = User::get_by_uuid(censored_post.author_uuid, conn)?;

        Ok(ChildlessPostData {
            post: censored_post,
            user,
        })

    }

    /// Gets all of the posts associated with a given user.
    pub fn get_posts_by_user(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<Vec<ChildlessPostData>> {
        use schema::posts::dsl::*;
        let user: User = User::get_by_uuid(user_uuid.0, conn)?;

        let user_posts: Vec<Post> = Post::belonging_to(&user)
            .order(created_date)
            .load::<Post>(conn)
            .map_err(Post::handle_error)?;

        return Ok(
            user_posts
                .into_iter()
                .map(|post| {
                    ChildlessPostData {
                        post,
                        user: user.clone(),
                    }
                })
                .collect(),
        );
    }


    /// Gets the user associated with a given post
    pub fn get_user_by_post(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<User> {
        use schema::posts::dsl::*;
        use schema::users::dsl::*;
        use schema::posts;

        let authors_uuid: Uuid = posts
            .find(post_uuid.0)
            .select(posts::author_uuid)
            .first::<Uuid>(conn)
            .map_err(Post::handle_error)?;

        users
            .find(authors_uuid)
            .first(conn)
            .map_err(User::handle_error)
    }

    /// Gets the first post associated with a thread.
    /// This post is identifed by it not having a parent id.
    /// All posts in a given thread that aren't root posts will have non-null parent ids.
    pub fn get_root_post(requested_thread_id: ThreadUuid, conn: &PgConnection) -> JoeResult<Post> {
        use schema::posts::dsl::*;
        use thread::Thread;

        let thread: Thread = Thread::get_by_uuid(requested_thread_id.0, conn)?;

        // Because this method is used in the context of a thread that could be immutable,
        // it should be subject to the locking mechanism.
        if thread.locked || thread.archived {
            return Err(WeekendAtJoesError::ThreadImmutable);
        }

        Post::belonging_to(&thread)
            .filter(
                parent_uuid.is_null(), // There should only be one thread that has a null parent, and that is the OP/root post
            )
            .first::<Post>(conn)
            .map_err(Post::handle_error)
    }

    pub fn get_individual_post(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        let post = Post::get_by_uuid(post_uuid.0, conn)?;
        let user = User::get_by_uuid(post.author_uuid, conn)?;
        Ok(ChildlessPostData { post, user })
    }


    /// Given the thread uuid, return a tree of posts.
    pub fn get_posts_in_thread(thread_uuid: ThreadUuid, conn: &PgConnection) -> JoeResult<PostData> {
        use schema::posts;
        use schema::posts::dsl::posts as posts_dsl;
        use std::collections::HashSet;

        let mut posts: Vec<Post> = posts_dsl
            .filter(posts::thread_uuid.eq(thread_uuid.0))
            .load::<Post>(conn)
            .map_err(Post::handle_error)?;

        if posts.len() == 0 {
            return Err(WeekendAtJoesError::NotFound { type_name: "Post" })
        }
        // We now know that there is at least one post.

        let mut user_uuids: Vec<Uuid> = posts.iter()
            .map(|post| post.author_uuid)
            .collect();
        // It isn't ideal to sort these, so this approach to deduplication works fine.
        let set: HashSet<_> = user_uuids.drain(..).collect(); // dedup
        user_uuids.extend(set.into_iter());

        let mut users: Vec<User> = Vec::with_capacity(user_uuids.len());

        for uuid in user_uuids {
            users.push(User::get_by_uuid(uuid, conn)?);
        }

        let users: HashMap<Uuid, User> = users.into_iter().map(|u| (u.uuid, u)).collect();

        // Remove the root from the list.
        let root: Vec<Post> = posts
            .drain_filter(|post| {
                if let None = post.parent_uuid {
                    true
                } else {
                    false
                }
            })
            .collect();
        // We are making the assumption that there is at least one post that meets the root criteria.
        // Practically speaking, there should be exactly one, but we rely on reasonable insertions
        // and modifications to enforce that.
        let root: Post = root.into_iter().next().ok_or(WeekendAtJoesError::InternalServerError)?;


        /// Recursive function to assemble posts out of the list of post data.
        /// It has a time complexity of O: n * log_n, but that is still better than talking to the database,
        /// as the constant time is too great there.
        fn assemble_posts (post: Post, posts: &mut Vec<Post>, users: &HashMap<Uuid, User>) -> PostData {
            let children: Vec<Post> = posts
                .drain_filter(|child_post: &mut Post| {
                    if let Some(parent_uuid) = child_post.parent_uuid {
                        parent_uuid == post.uuid
                    } else {
                        false
                    }
                })
                .collect();

            // Recurse
            let children: Vec<PostData> = children.into_iter().map(|child_post| {
                assemble_posts(child_post, posts, users)
            }).collect();

            let user: User = users.get(&post.author_uuid).cloned().expect("The user at the uuid should exist");

            PostData {
                post,
                user,
                children
            }
        }
        Ok(assemble_posts(root, &mut posts, &users))
    }

    /// Gets the post at the given UUID and all of its children.
    ///
    /// The current implementation has overhead as it requires getting all the posts in a thread
    /// and then pruning it down to the desired subsection of the tree.
    pub fn get_post_and_children(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<PostData> {
        // Get the post so we can get the thread
        let post = Post::get_by_uuid(post_uuid.0, conn)?;
        let thread_uuid: ThreadUuid = ThreadUuid(post.thread_uuid);
        let post_tree: PostData = Post::get_posts_in_thread(thread_uuid, conn)?;

        /// Recursive inner function to prune the tree to find the desired node among the larger post_tree.
        fn find_post_node(post_uuid: Uuid, post_tree: PostData) -> Option<PostData> {
            if post_tree.post.uuid == post_uuid {
                return Some(post_tree);
            } else {
                for child in post_tree.children {
                    if let Some(found_root) = find_post_node(post_uuid, child) {
                        return Some(found_root);
                    }
                }
                return None;
            }
        }

        let desired_post_node = find_post_node(post_uuid.0, post_tree)
            .expect("The post should be inside the tree, because it is known to exist at the start of this function.");
        Ok(desired_post_node)

    }


}
