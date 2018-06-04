use schema::posts;
use chrono::NaiveDateTime;
use db::user::User;
use db::thread::Thread;
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


#[derive(Debug, Clone, Identifiable, Associations, Queryable, CrdUuid, ErrorHandler)]
#[insertable = "NewPost"]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Thread, foreign_key = "thread_id")]
#[belongs_to(Post, foreign_key = "parent_id")]
#[table_name = "posts"]
pub struct Post {
    /// Primary Key
    pub id: Uuid,
    /// The Foreign Key of the thread the post belongs to.
    pub thread_id: Uuid,
    /// The Foreign Key of the user that created the post.
    pub author_id: Uuid,
    /// The Foreign Key of the post to which this post is replying to.
    pub parent_id: Option<Uuid>,
    /// The timestamp of when the post was created.
    pub created_date: NaiveDateTime,
    /// If the post was edited, the most recent edit time will be attached to the post.
    pub modified_date: Option<NaiveDateTime>,
    /// The content of the post. This may be rendered with markdown or a subset thereof.
    pub content: String,
    /// If the post has been censored, it will not be immediately viewabe by people viewing the thread.
    pub censored: bool,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "posts"]
pub struct NewPost {
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub parent_id: Option<Uuid>, // this will always be None, try removing this.
    pub created_date: NaiveDateTime,
    pub content: String,
    pub censored: bool,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug, Identifiable)]
#[table_name = "posts"]
pub struct EditPostChangeset {
    pub id: Uuid,
    pub modified_date: NaiveDateTime,
    pub content: String,
}


pub struct PostData {
    pub post: Post,
    pub user: User,
    pub children: Vec<PostData>,
}

pub struct ChildlessPostData {
    pub post: Post,
    pub user: User,
}

impl Post {
    /// Applies the EditPostChangeset to the post.
    /// If the thread is locked, the post cannot be modified
    pub fn modify_post(edit_post_changeset: EditPostChangeset, thread_uuid: ThreadUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        //        use schema::posts;

        let target_thread = Thread::get_by_uuid(thread_uuid.0, conn)?;
        if target_thread.locked {
            return Err(WeekendAtJoesError::ThreadLocked);
        }


        let modified_post: Post = edit_post_changeset
            .save_changes(conn)
            .map_err(Post::handle_error)?;
        let user = User::get_by_uuid(modified_post.author_id, conn)?;
        Ok(ChildlessPostData {
            post: modified_post,
            user,
        })
    }


    /// Creates a post, and also gets the associated author for the post.
    pub fn create_and_get_user(new_post: NewPost, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        let post: Post = Post::create(new_post, conn)?;
        let user: User = User::get_by_uuid(post.author_id, conn)?;
        Ok(ChildlessPostData { post, user })
    }

    /// Censors the post, preventing users from seeing it by default.
    pub fn censor_post(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        use schema::posts::dsl::*;
        use schema::posts;

        let m_post_uuid: Uuid = post_uuid.0;

        let censored_post: Post = diesel::update(posts::table)
            .filter(posts::id.eq(m_post_uuid))
            .set(censored.eq(true))
            .get_result(conn)
            .map_err(Post::handle_error)?;
        let user = User::get_by_uuid(censored_post.author_id, conn)?;

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


        // TODO consider using a select to just pull out the author id
        let post: Post = posts
            .find(post_uuid.0)
            .first::<Post>(conn)
            .map_err(Post::handle_error)?;

        users
            .find(post.author_id)
            .first(conn)
            .map_err(User::handle_error)
    }

    /// Gets the first post associated with a thread.
    /// This post is identifed by it not having a parent id.
    /// All posts in a given thread that aren't root posts will have non-null parent ids.
    pub fn get_root_post(requested_thread_id: ThreadUuid, conn: &PgConnection) -> JoeResult<Post> {
        use schema::posts::dsl::*;
        use db::thread::Thread;

        let thread: Thread = Thread::get_by_uuid(requested_thread_id.0, conn)?;

        Post::belonging_to(&thread)
            .filter(
                parent_id.is_null(), // There should only be one thread that has a null parent, and that is the OP/root post
            )
            .first::<Post>(conn)
            .map_err(Post::handle_error)
    }

    pub fn get_individual_post(post_uuid: PostUuid, conn: &PgConnection) -> JoeResult<ChildlessPostData> {
        let post = Post::get_by_uuid(post_uuid.0, conn)?;
        let user = User::get_by_uuid(post.author_id, conn)?;
        Ok(ChildlessPostData { post, user })
    }

    /// Gets all of the children for a post and assembles the tree with the `self` post as the root node.
    /// This will make recursive calls into the database.
    /// This method should be the target of significant scrutiny.
    pub fn get_post_data(self, conn: &PgConnection) -> JoeResult<PostData> {
        let user: User = User::get_by_uuid(self.author_id, conn)?;
        let children: Vec<Post> = self.get_post_children(conn)?; // gets the children
        // turns the children into PostData
        let children: Vec<PostData> = children
            .into_iter()
            .map(|child| child.get_post_data(conn))
            .collect::<Result<Vec<PostData>, WeekendAtJoesError>>()?;

        Ok(PostData {
            post: self,
            user,
            children,
        })
    }

    /// Gets all of the posts that belong to the post.
    pub fn get_post_children(&self, conn: &PgConnection) -> Result<Vec<Post>, WeekendAtJoesError> {
        Post::belonging_to(self)
            .load::<Post>(conn)
            .map_err(Post::handle_error)
    }
}
