use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use warp;
use crate::json_body_filter;
use crate::jwt::normal_user_filter;
use crate::db_integration::db_filter;
use wire::answer::NewAnswerRequest;
use identifiers::user::UserUuid;
use identifiers::question::QuestionUuid;
use db::Conn;
use db::Question;
use db::User;
use db::CreatableUuid;
use db::RetrievableUuid;
use db::answer::AnswerData;
use db::answer::NewAnswer;
use db::answer::Answer;
use crate::error::Error;
use wire::answer::AnswerResponse;


pub fn answer_api() -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Answer API");
    warp::path("answer")
        .and(
            answer_question()
        )
        .with(warp::log("answer"))
        .boxed()
}

fn answer_question() -> BoxedFilter<(impl Reply,)> {
    warp::post2()
        .and(json_body_filter(16))
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: NewAnswerRequest, user_uuid: UserUuid, conn: Conn|{
            let new_answer: NewAnswerRequest = request;
            let question_uuid: QuestionUuid = new_answer.question_uuid.clone(); // spurious clone

            let new_answer: NewAnswer = NewAnswer::attach_user_id(new_answer, user_uuid);
            let answer_user: User = User::get_by_uuid(new_answer.author_uuid, &conn)
                .map_err(Error::convert_and_reject)?;


            Question::put_question_on_floor(question_uuid, &conn)
                .map_err(Error::convert_and_reject)?;

            Answer::create(new_answer, &conn)
                .map(|answer| {
                    AnswerData {
                        answer,
                        user: answer_user,
                    }
                })
                .map(crate::convert_and_json::<AnswerData,AnswerResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}