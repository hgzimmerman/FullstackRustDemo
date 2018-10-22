use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
//use crate::db_integration::s.db.clone();
use db::Conn;
use crate::util::convert_and_json;
use crate::util::convert_vector_and_json;
use crate::util::json_body_filter;
use identifiers::user::UserUuid;
use crate::jwt::normal_user_filter;
use wire::post::NewPostRequest;
use db::Post;
use wire::post::PostResponse;
use db::post::NewPost;
use db::post::ChildlessPostData;
use wire::post::EditPostRequest;
use identifiers::thread::ThreadUuid;
use db::RetrievableUuid;
use db::post::EditPostChangeset;
use crate::jwt::moderator_user_filter;
use identifiers::post::PostUuid;
use crate::logging::log_attach;
use crate::logging::HttpMethod;
use crate::uuid_integration::uuid_wrap_filter;
use crate::state::State;
use pool::PooledConn;


pub fn post_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Post API");
    let api = create_post(s)
        .or(edit_post(s))
        .or(censor_post(s))
        .or(get_posts_by_user(s))
        ;

    warp::path("post")
        .and(api)
        .with(warp::log("post"))
        .boxed()
}


pub fn create_post(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "post/");

    warp::post2()
        .and(json_body_filter(12))
        .and(normal_user_filter())
        .and(s.db.clone())
        .and_then(|request: NewPostRequest, user_uuid: UserUuid, conn: PooledConn| {
            // check if token user id matches the request user id.
            // This prevents users from creating posts under other user's names.
            let new_post: NewPost = request.into();
            if new_post.author_uuid != user_uuid.0 {
                return Error::BadRequest.reject()
            }
            Post::create_and_get_user(new_post, &conn)
                .map(convert_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)

        })
        .boxed()
}

pub fn edit_post(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Put, "post/");

    warp::put2()
        .and(json_body_filter(12))
        .and(normal_user_filter())
        .and(s.db.clone())
        .and_then(|request: EditPostRequest, user_uuid: UserUuid, conn: PooledConn|{
             // Prevent editing other users posts
             let existing_post = Post::get_by_uuid(request.uuid.0, &conn).map_err(Error::convert_and_reject)?;
             if user_uuid.0 != existing_post.author_uuid {
                 return Error::BadRequest.reject()
             }

             let edit_post_request: EditPostRequest = request;
             let edit_post_changeset: EditPostChangeset = edit_post_request.clone().into();
             let thread_id: ThreadUuid = edit_post_request.thread_uuid;
             Post::modify_post(edit_post_changeset, thread_id, user_uuid, &conn)
                .map(convert_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

pub fn censor_post(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Put, "post/censor");

    warp::put2()
        .and(warp::path("censor"))
        .and(uuid_wrap_filter())
//        .and(warp::path::param::<PosUuid>())
        .and(moderator_user_filter())
        .and(s.db.clone())
        .and_then(|post_uuid: PostUuid, _user: UserUuid, conn: PooledConn| {
            Post::censor_post(post_uuid, &conn)
                .map(convert_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

pub fn get_posts_by_user(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "post/users_posts");

    warp::get2()
        .and(warp::path("users_posts"))
        .and(uuid_wrap_filter())
        .and(s.db.clone())
        .and_then(|user_uuid: UserUuid, conn: PooledConn| {
            Post::get_posts_by_user(user_uuid, &conn)
                .map(convert_vector_and_json::<ChildlessPostData, PostResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}