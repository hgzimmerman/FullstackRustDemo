use crate::schema::posts;
use crate::schema::post_upvotes;
use crate::schema::post_downvotes;
use chrono::NaiveDateTime;
use crate::user::User;
use crate::thread::Thread;
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
    pub votes: VoteCounts,
    pub children: Vec<PostData>,
}

#[derive(Debug, Clone)]
pub struct ChildlessPostData {
    pub post: Post,
    pub user: User,
    pub votes: VoteCounts
}






#[derive(Debug, Clone, PartialEq, Identifiable, Associations, Queryable)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[belongs_to(Post, foreign_key = "post_uuid")]
#[table_name = "post_upvotes"]
struct PostUpvote {
    /// Primary Key
    pub uuid: Uuid,
    /// Foreign Key
    pub post_uuid: Uuid,
    /// Foreign Key
    pub user_uuid: Uuid
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "post_upvotes"]
struct NewUpvote {
    pub post_uuid: Uuid,
    pub user_uuid: Uuid
}

#[derive(Debug, Clone, PartialEq, Identifiable, Associations, Queryable)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[belongs_to(Post, foreign_key = "post_uuid")]
#[table_name = "post_downvotes"]
struct PostDownvote {
    /// Primary Key
    pub uuid: Uuid,
    /// Foreign Key
    pub post_uuid: Uuid,
    /// Foreign Key
    pub user_uuid: Uuid
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "post_downvotes"]
struct NewDownvote {
    pub post_uuid: Uuid,
    pub user_uuid: Uuid
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vote {
    pub post_uuid: PostUuid,
    pub user_uuid: UserUuid,
}

pub enum PostVote {
    Up(Vote),
    Down(Vote)
}

#[derive(Clone, Debug, PartialEq)]
pub struct VoteCounts {
    pub up: i64,
    pub down: i64,
    pub user_voted_up: bool,
    pub user_voted_down: bool
}

impl From<Vote> for NewDownvote {
    fn from(f: Vote) -> Self {
        NewDownvote {
            post_uuid: f.post_uuid.0,
            user_uuid: f.user_uuid.0
        }
    }
}
impl From<Vote> for NewUpvote {
    fn from(f: Vote) -> Self {
        NewUpvote {
            post_uuid: f.post_uuid.0,
            user_uuid: f.user_uuid.0
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

impl From<ChildlessPostData> for PostData {
    fn from(childless: ChildlessPostData) -> PostData {
        PostData {
            post: childless.post,
            user: childless.user,
            votes: childless.votes,
            children: vec![],
        }
    }
}



impl Post {
    /// Applies the EditPostChangeset to the post.
    /// If the thread is locked, the post cannot be modified
    pub fn modify_post(edit_post_changeset: EditPostChangeset, thread_uuid: ThreadUuid, user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        //        use schema::posts;

        let target_thread: Thread = Thread::get_by_uuid(thread_uuid.0, conn)?;
        if target_thread.locked || target_thread.archived {
            return Err(WeekendAtJoesError::ThreadImmutable);
        }


        let modified_post: Post = edit_post_changeset
            .save_changes(conn)
            .map_err(Post::handle_error)?;

        let votes: VoteCounts = Post::get_vote_counts(&modified_post, user_uuid, conn)?;

        let user = User::get_by_uuid(modified_post.author_uuid, conn)?;
        Ok(ChildlessPostData {
            post: modified_post,
            user,
            votes
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
        let user_uuid = UserUuid(post.author_uuid);
        let votes: VoteCounts = Post::get_vote_counts(&post, user_uuid, conn)?;
        let post_data = ChildlessPostData {
            post,
            user,
            votes
        };
        Ok(post_data)
    }

    /// Censors the post, preventing users from seeing it by default.
    pub fn censor_post(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        use crate::schema::posts::dsl::*;
        use crate::schema::posts;

        let m_post_uuid: Uuid = post_uuid.0;

        let censored_post: Post = diesel::update(posts::table)
            .filter(posts::uuid.eq(m_post_uuid))
            .set(censored.eq(true))
            .get_result(conn)
            .map_err(Post::handle_error)?;
        let user = User::get_by_uuid(censored_post.author_uuid, conn)?;
        let user_uuid = UserUuid(user.uuid);
        let votes = Post::get_vote_counts(&censored_post, user_uuid,conn)?;

        Ok(ChildlessPostData {
            post: censored_post,
            user,
            votes
        })

    }

    /// Gets all of the posts associated with a given user.
    pub fn get_posts_by_user(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<Vec<ChildlessPostData>> {
        use crate::schema::posts::dsl::*;
        let user: User = User::get_by_uuid(user_uuid.0, conn)?;

        let user_posts: Vec<Post> = Post::belonging_to(&user)
            .order(created_date)
            .load::<Post>(conn)
            .map_err(Post::handle_error)?;

        let votes = Post::get_votes_for_posts(&user_posts, Some(user_uuid), conn)?;
        let posts_and_votes: Vec<(Post, VoteCounts)> = user_posts.into_iter().zip(votes.into_iter()).collect();

        return Ok(
            posts_and_votes
                .into_iter()
                .map(|p_and_c: (Post, VoteCounts)| {
                    let (post, votes) = p_and_c;
                    ChildlessPostData {
                        post,
                        user: user.clone(),
                        votes
                    }
                })
                .collect(),
        );
    }


    /// Gets the user associated with a given post
    pub fn get_user_by_post(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<User> {
        use crate::schema::posts::dsl::*;
        use crate::schema::users::dsl::*;
        use crate::schema::posts;

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
        use crate::schema::posts::dsl::*;
        use crate::thread::Thread;

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

    pub fn get_individual_post(post_uuid: PostUuid, user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        let post = Post::get_by_uuid(post_uuid.0, conn)?;
        let user = User::get_by_uuid(post.author_uuid, conn)?;
        let votes = Post::get_vote_counts(&post, user_uuid,conn)?;
        Ok(ChildlessPostData { post, user, votes })
    }


    /// Given the thread uuid, return a tree of posts.
    pub fn get_posts_in_thread(thread_uuid: ThreadUuid, user_uuid: Option<UserUuid>, conn: &PgConnection) -> JoeResult<PostData> {
        use crate::schema::posts;
        use crate::schema::posts::dsl::posts as posts_dsl;
        use std::collections::HashSet;

        let posts: Vec<Post> = posts_dsl
            .filter(posts::thread_uuid.eq(thread_uuid.0))
            .load::<Post>(conn)
            .map_err(Post::handle_error)?;


        if posts.len() == 0 {
            return Err(WeekendAtJoesError::NotFound { type_name: "Post" })
        }
        // We now know that there is at least one post.

        let votes = Post::get_votes_for_posts(&posts, user_uuid, conn)?;
        let mut posts_and_votes: Vec<(Post, VoteCounts)> = posts.into_iter().zip(votes.into_iter()).collect();

        let mut user_uuids: Vec<Uuid> = posts_and_votes.iter()
            .map(|post| post.0.author_uuid)
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
        let root: Vec<(Post, VoteCounts)> = posts_and_votes
            .drain_filter(|post_and_votes| {
                let post: &Post = &post_and_votes.0;
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
        let root: (Post, VoteCounts) = root.into_iter().next().ok_or(WeekendAtJoesError::InternalServerError)?;


        /// Recursive function to assemble posts out of the list of post data.
        /// It has a time complexity of O: n * log_n, but that is still better than talking to the database,
        /// as the constant time is too great there.
        fn assemble_posts (post_and_votes: (Post, VoteCounts), posts_and_votes: &mut Vec<(Post, VoteCounts)>, users: &HashMap<Uuid, User>) -> PostData {

            let children: Vec<(Post, VoteCounts)> = posts_and_votes
                .drain_filter(|child_post_and_votes: &mut (Post, VoteCounts)| {
                    let child_post: &Post = &child_post_and_votes.0;
                    if let Some(parent_uuid) = child_post.parent_uuid {
                        parent_uuid == post_and_votes.0.uuid
                    } else {
                        false
                    }
                })
                .collect();

            // Recurse
            let children: Vec<PostData> = children.into_iter().map(|child_post: (Post, VoteCounts)| {
                assemble_posts(child_post, posts_and_votes, users)
            }).collect();

            let user: User = users
                .get(&post_and_votes.0.author_uuid)
                .cloned()
                .expect("The user at the uuid should exist");

            let (post, votes) = post_and_votes;

            PostData {
                post,
                user,
                votes,
                children,
            }
        }

        Ok(assemble_posts(root, &mut posts_and_votes, &users))
    }

    /// Gets the post at the given UUID and all of its children.
    ///
    /// The current implementation has overhead as it requires getting all the posts in a thread
    /// and then pruning it down to the desired subsection of the tree.
    pub fn get_post_and_children(post_uuid: PostUuid, user_uuid: Option<UserUuid>, conn: &PgConnection) -> JoeResult<PostData> {
        // Get the post so we can get the thread
        let post = Post::get_by_uuid(post_uuid.0, conn)?;
        let thread_uuid: ThreadUuid = ThreadUuid(post.thread_uuid);
        let post_tree: PostData = Post::get_posts_in_thread(thread_uuid, user_uuid, conn)?;


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

    /// Add a vote record to a post.
    pub fn vote(vote: PostVote, conn: &PgConnection) -> JoeResult<()> {
        use crate::schema::post_upvotes;
        use crate::schema::post_downvotes;
        use diesel::dsl::exists;
        use diesel::select;

        let v: &Vote = match vote {
            PostVote::Up(ref vote) => vote,
            PostVote::Down(ref vote) => vote
        };

        let upvote_exists: bool = select(
            exists(
                post_upvotes::table
                    .filter(post_upvotes::user_uuid.eq(v.user_uuid.0))
                    .filter(post_upvotes::post_uuid.eq(v.post_uuid.0))
                )
            )
            .get_result(conn)
            .map_err(Post::handle_error)?;


        let downvote_exists: bool = select(
            exists(
                post_downvotes::table
                    .filter(post_downvotes::user_uuid.eq(v.user_uuid.0))
                    .filter(post_downvotes::post_uuid.eq(v.post_uuid.0))
                )
            )
            .get_result(conn)
            .map_err(Post::handle_error)?;

        match vote {
            PostVote::Up(vote) => {
                if downvote_exists {
                    Post::remove_downvote(vote.user_uuid, vote.post_uuid, conn)?
                }
                if !upvote_exists {
                    // Insert the new upvote
                    let upvote: NewUpvote = vote.into();
                    return diesel::insert_into(post_upvotes::table)
                        .values(upvote)
                        .execute(conn)
                        .map_err(Post::handle_error)
                        .map(|_| ())
                } else {
                    return Err(WeekendAtJoesError::BadRequest)
                }
            },
            PostVote::Down(vote) => {
                if upvote_exists {
                    Post::remove_upvote(vote.user_uuid, vote.post_uuid, conn)?
                }
                if !downvote_exists {
                    // Add the new downvote
                    let downvote: NewDownvote = vote.into();
                    return diesel::insert_into(post_downvotes::table)
                        .values(downvote)
                        .execute(conn)
                        .map_err(Post::handle_error)
                        .map(|_| ());
                } else {
                    return  Err(WeekendAtJoesError::BadRequest)
                }
            }
        };

    }

    /// Removes an upvote from a post.
    fn remove_upvote(user_uuid: UserUuid, post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<()> {
        use crate::schema::post_upvotes;

        let target = post_upvotes::table
            .filter(post_upvotes::user_uuid.eq(user_uuid.0))
            .filter(post_upvotes::post_uuid.eq(post_uuid.0));
        diesel::delete(target)
            .execute(conn)
            .map_err(Post::handle_error)
            .map(|_| ())
    }
    /// Removes a downvote from a post.
    fn remove_downvote(user_uuid: UserUuid, post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<()> {
        use crate::schema::post_downvotes;

        let target = post_downvotes::table
            .filter(post_downvotes::user_uuid.eq(user_uuid.0))
            .filter(post_downvotes::post_uuid.eq(post_uuid.0));
        diesel::delete(target)
            .execute(conn)
            .map_err(Post::handle_error)
            .map(|_| ())
    }


    /// Remove any vote for the post
    pub fn revoke_vote(user_uuid: UserUuid, post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<()> {
        let x = Post::remove_upvote(user_uuid, post_uuid, conn);
        let y = Post::remove_downvote(user_uuid, post_uuid, conn);

        x?;
        y
    }

    /// Gets the vote counts for a single post.
    pub fn get_vote_counts(post: &Post, user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<VoteCounts> {
        use crate::schema::post_upvotes;
        use crate::schema::post_downvotes;
        use diesel::dsl::exists;
        use diesel::select;

        let up: i64 = PostUpvote::belonging_to(post)
            .count()
            .get_result(conn)
            .map_err(Post::handle_error)?;
        let down: i64 = PostDownvote::belonging_to(post)
            .count()
            .get_result(conn)
            .map_err(Post::handle_error)?;


        let user_voted_up: bool = select(
            exists(
                post_upvotes::table
                    .filter(post_upvotes::user_uuid.eq(user_uuid.0))
                    .filter(post_upvotes::post_uuid.eq(post.uuid))
                )
            )
            .get_result(conn)
            .map_err(Post::handle_error)?;

        let user_voted_down: bool = select(
            exists(
                post_downvotes::table
                    .filter(post_downvotes::user_uuid.eq(user_uuid.0))
                    .filter(post_downvotes::post_uuid.eq(post.uuid))
                )
            )
            .get_result(conn)
            .map_err(Post::handle_error)?;

        let counts: VoteCounts = VoteCounts {
            up,
            down,
            user_voted_up,
            user_voted_down
        };
        Ok(counts)
    }

    // TODO make this take a reference to a vector of posts, then only return the votes, not a tuple.
    /// Given a vector of posts, make a request to get the vote counts for each and associate the counts with the posts.
    pub fn get_votes_for_posts(posts: &Vec<Post>, user_uuid: Option<UserUuid>, conn: &PgConnection) -> JoeResult<Vec<VoteCounts>> {
        use diesel::GroupedBy;

        match user_uuid {
            Some(user_uuid) => {
                let up_counts: Vec<(i64, bool)> = PostUpvote::belonging_to(posts)
                    .load(conn)
                    .map_err(Post::handle_error)?
                    .grouped_by(&posts)
                    .into_iter()
                    .map(|l: Vec<PostUpvote>| {
                        let user_voted_up: bool = l.iter().any(|x| x.user_uuid == user_uuid.0 );
                        let count = l.len() as i64;
                        (count, user_voted_up)
                    })
                    .collect();
                let down_counts: Vec<(i64, bool)> = PostDownvote::belonging_to(posts)
                    .load(conn)
                    .map_err(Post::handle_error)?
                    .grouped_by(&posts)
                    .into_iter()
                    .map(|l: Vec<PostDownvote>| {
                        let user_voted_up: bool = l.iter().any(|x| x.user_uuid == user_uuid.0 );
                        let count = l.len() as i64;
                        (count, user_voted_up)
                    })
                    .collect();

                let counts: Vec<VoteCounts> = up_counts
                    .into_iter()
                    .zip(down_counts.into_iter())
                    .map(|x: ((i64, bool), (i64, bool))| VoteCounts {
                        up: (x.0).0,
                        down: (x.1).0,
                        user_voted_up: (x.0).1,
                        user_voted_down: (x.1).1
                    })
                    .collect();
                Ok(counts)
            }
            None => {
                let up_counts: Vec<i64> = PostUpvote::belonging_to(posts)
                    .load(conn)
                    .map_err(Post::handle_error)?
                    .grouped_by(&posts)
                    .into_iter()
                    .map(|l: Vec<PostUpvote>| l.len() as i64)
                    .collect();
                let down_counts: Vec<i64> = PostDownvote::belonging_to(posts)
                    .load(conn)
                    .map_err(Post::handle_error)?
                    .grouped_by(&posts)
                    .into_iter()
                    .map(|l: Vec<PostDownvote>| l.len() as i64)
                    .collect();
                let counts: Vec<VoteCounts> = up_counts
                    .into_iter()
                    .zip(down_counts.into_iter())
                    .map(|x: (i64, i64)| VoteCounts {
                        up: x.0,
                        down: x.1,
                        user_voted_up: false,
                        user_voted_down: false
                    })
                    .collect();
                Ok(counts)
            }
        }


//        let posts_and_vote_counts: Vec<(Post, VoteCounts)> = posts.into_iter().zip(counts.into_iter()).collect();
    }


}
