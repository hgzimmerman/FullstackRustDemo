use warp::{
    filters::BoxedFilter,
    Filter,
    reply::Reply,
    self
};
use crate::{
    state::jwt::normal_user_filter,
    util::json_body_filter,
    logging::log_attach,
    logging::HttpMethod,
    util::convert_and_json,
    state::State
};
use wire::{
    answer::{
        NewAnswerRequest,
        AnswerResponse
    }
};
use identifiers::{
    user::UserUuid,
    question::QuestionUuid
};
use db::{
    Question,
    User,
    answer::{
        AnswerData,
        NewAnswer,
        Answer
    }
};
use error::Error;
use pool::PooledConn;


pub fn answer_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Answer API");
    warp::path("answer")
        .and(
            answer_question(s)
        )
        .with(warp::log("answer"))
        .boxed()
}

fn answer_question(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "answer/");

    warp::post2()
        .and(json_body_filter(16))
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewAnswerRequest, user_uuid: UserUuid, conn: PooledConn|{
            let new_answer: NewAnswerRequest = request;
            let question_uuid: QuestionUuid = new_answer.question_uuid.clone(); // spurious clone

            let new_answer: NewAnswer = NewAnswer::attach_user_id(new_answer, user_uuid);
            let author_uuid = UserUuid(new_answer.author_uuid);
            let answer_user: User = User::get_user(author_uuid, &conn)
                .map_err(Error::simple_reject)?;


            Question::put_question_on_floor(question_uuid, &conn)
                .map_err(Error::simple_reject)?;

            Answer::create_answer(new_answer, &conn)
                .map(|answer| {
                    AnswerData {
                        answer,
                        user: answer_user,
                    }
                })
                .map(convert_and_json::<AnswerData,AnswerResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}