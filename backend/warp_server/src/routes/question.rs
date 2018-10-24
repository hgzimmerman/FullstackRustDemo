use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use crate::error::Error;
//use crate::db_integration::s.db.clone();
//use db::Conn;
use uuid::Uuid;
use crate::util::convert_and_json;
use crate::util::convert_vector_and_json;
use crate::util::json_body_filter;
use identifiers::user::UserUuid;
use crate::util::query_uuid;
use db::Question;
use identifiers::bucket::BucketUuid;
use wire::question::QuestionResponse;
use db::question::QuestionData;
use crate::uuid_integration::uuid_filter;
use identifiers::question::QuestionUuid;
use crate::state::jwt::normal_user_filter;
use wire::question::NewQuestionRequest;
use db::Bucket;
use db::question::NewQuestion;
use crate::logging::log_attach;
use crate::logging::HttpMethod;
use crate::uuid_integration::uuid_wrap_filter;
use crate::state::State;
use pool::PooledConn;


pub fn question_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Question API");
    let api = get_questions_for_bucket(s)
        .or(get_question(s))
        .or(create_question(s))
        .or(get_random_question(s))
        .or(get_questions_for_bucket(s))
        .or(delete_question(s))
        .or(put_question_back_in_bucket(s))
        .or(questions_in_bucket(s))
        ;

    warp::path("question")
        .and(api)
        .with(warp::log("question"))
        .boxed()
}


pub fn get_questions_for_bucket(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "question?bucket_uuid=<uuid>");

    warp::get2()
        .and(query_uuid("bucket_uuid"))
        .and(s.db.clone())
        .and_then(|bucket_uuid: Uuid, conn: PooledConn| {
            let bucket_uuid = BucketUuid(bucket_uuid);
            Question::get_questions_for_bucket(bucket_uuid, &conn)
                .map(convert_vector_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}


fn get_random_question(s: &State) -> BoxedFilter<(impl Reply,)> {

//    log_attach(HttpMethod::Post, "bucket/");
    log_attach(HttpMethod::Get, "question/random_question?bucket_uuid=<uuid>");

    warp::get2()
        .and(warp::path("random_question"))
        .and(query_uuid("bucket_uuid"))
        .and(s.db.clone())
        .and_then(|bucket_uuid: Uuid, conn: PooledConn| {
            let bucket_uuid = BucketUuid(bucket_uuid);
            Question::get_random_question(bucket_uuid, &conn)
                .map(convert_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}
fn get_question(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "question/<uuid>");

    warp::get2()
        .and(uuid_wrap_filter())
        .and(s.db.clone())
        .and_then(|question_uuid: QuestionUuid, conn: PooledConn| {
            Question::get_full_question(question_uuid, &conn)
                .map(convert_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()

}


// TODO there should be a variant that doesn't require auth.
fn create_question(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "question/");

    warp::post2()
        .and(json_body_filter(12))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewQuestionRequest, user_uuid: UserUuid, conn: PooledConn| {
            let bucket_uuid: BucketUuid = request.bucket_uuid;
            let is_approved  = Bucket::is_user_approved(user_uuid, bucket_uuid, &conn);
            if !is_approved {
                return Error::BadRequest.reject()
            }

            let new_question: NewQuestion = NewQuestion::attach_user_id(request, user_uuid);

            Question::create_data(new_question, &conn)
                .map(convert_and_json::<QuestionData, QuestionResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn delete_question(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Delete, "question/<uuid>");

    warp::delete2()
        .and(uuid_filter())
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|question_uuid: Uuid, _user_uuid: UserUuid, conn: PooledConn | {
            let question_uuid = QuestionUuid(question_uuid);
            Question::delete_question(question_uuid.clone(), &conn)
                .map_err(Error::convert_and_reject)
                .map(|_| warp::reply::json(&question_uuid))
        })
        .boxed()
}

fn put_question_back_in_bucket(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Put, "question/<uuid>/into_bucket/");

    warp::put2()
        .and(uuid_filter())
        .and(warp::path("into_bucket"))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|question_uuid: Uuid, _user_uuid: UserUuid, conn: PooledConn | {
            let question_uuid = QuestionUuid(question_uuid);
            Question::put_question_in_bucket(question_uuid, &conn)
                .map_err(Error::convert_and_reject)
                .map(|_| warp::reply::json(&question_uuid))
        })
        .boxed()
}


fn questions_in_bucket(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "question/quantity_in_bucket?bucket_uuid=<uuid>");

    warp::get2()
        .and(warp::path("quantity_in_bucket"))
        .and(query_uuid("bucket_uuid"))
        .and(s.db.clone())
        .and_then(|bucket_uuid: Uuid, conn: PooledConn| {
            let bucket_uuid = BucketUuid(bucket_uuid);
            Question::get_number_of_questions_in_bucket(bucket_uuid, &conn)
                .map(convert_and_json::<i64, i64>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}