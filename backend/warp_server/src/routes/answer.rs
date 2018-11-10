use crate::{
    logging::{
        log_attach,
        HttpMethod,
    },
    state::{
        State,
        jwt::optional_normal_user_filter
    },
    util::{
        convert_and_json,
        json_body_filter,
    },
};
use db::{
    answer::{
        Answer,
        AnswerData,
        NewAnswer,
    },
    Question,
    User,
};
use error::Error;
use identifiers::{
    question::QuestionUuid,
    user::UserUuid,
};
use pool::PooledConn;
use warp::{
    self,
    filters::BoxedFilter,
    reply::Reply,
    Filter,
};
use wire::answer::{
    AnswerResponse,
    NewAnswerRequest,
};

pub fn answer_api(s: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Answer API");
    warp::path("answer")
        .and(answer_question(s))
        .with(warp::log("answer"))
        .boxed()
}

fn answer_question(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "answer/");

    warp::post2()
        .and(json_body_filter(16))
        .and(optional_normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: NewAnswerRequest, user_uuid: Option<UserUuid>, conn: PooledConn| {
            let new_answer: NewAnswerRequest = request;
            let question_uuid: QuestionUuid = new_answer.question_uuid.clone(); // spurious clone

            let new_answer: NewAnswer = NewAnswer::attach_user_id(new_answer, user_uuid);

            let answer_author: Option<User> = new_answer.author_uuid
                .map(UserUuid)
                .map(|author_uuid| User::get_user(author_uuid, &conn).map_err(Error::simple_reject))
                .transpose()?;
//            let answer_user: Option<User> = User::get_user(author_uuid, &conn).map_err(Error::simple_reject)?;

            Question::put_question_on_floor(question_uuid, &conn).map_err(Error::simple_reject)?;

            Answer::create_answer(new_answer, &conn)
                .map(|answer| AnswerData {
                    answer,
                    user: answer_author,
                })
                .map(convert_and_json::<AnswerData, AnswerResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}
