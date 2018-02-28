use rocket_contrib::Json;
use routes::Routable;
use rocket::Route;

use db::Retrievable;
use db::question::*;
use db::answer::Answer;
use error::WeekendAtJoesError;
use db::user::User;
use db::Conn;
use requests_and_responses::question::*;
use requests_and_responses::answer::*;
use auth::user_authorization::*;
use routes::answer::AnswerData;

pub struct QuestionData(pub (Question, User, Vec<Answer>));
impl From<QuestionData> for QuestionResponse {
    fn from(tuple: QuestionData) -> QuestionResponse {
        let (question, user, answers) = tuple.0;
        QuestionResponse {
            id: question.id,
            bucket_id: question.bucket_id,
            question_text: question.question_text,
            author: user.clone().into(),
            answers: answers
                .into_iter()
                .map(|a| AnswerResponse::from(AnswerData((a, user.clone()))))
                .collect(),
        }
    }
}

impl From<NewQuestionRequest> for NewQuestion {
    fn from(request: NewQuestionRequest) -> NewQuestion {
        NewQuestion {
            bucket_id: request.bucket_id,
            author_id: request.author_id,
            question_text: request.question_text,
        }
    }
}

#[get("/questions_in_bucket/<bucket_id>")]
fn get_questions_for_bucket(bucket_id: i32, conn: Conn) -> Result<Json<Vec<QuestionResponse>>, WeekendAtJoesError> {

    Question::get_questions_for_bucket(bucket_id, &conn)
        .map(|groups| {
            groups
                .into_iter()
                .flat_map(|group: (User, Vec<(Question, Vec<Answer>)>)| {
                    let user: User = group.0;
                    let question_groups: Vec<(Question, Vec<Answer>)> = group.1;
                    question_groups
                        .into_iter()
                        .map(|question_group: (Question, Vec<Answer>)| {
                            let (question, answers) = question_group;
                            QuestionResponse::from(QuestionData((question, user.clone(), answers)))
                        })
                        .collect::<Vec<QuestionResponse>>()
                })
                .collect()
        })
        .map(Json)
}

#[get("/random_question/<bucket_id>")]
fn get_random_unanswered_question(bucket_id: i32, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    Question::get_random_unanswered_question(bucket_id, &conn)
        .map(|group: (Question, User)| QuestionResponse::from(QuestionData((group.0, group.1, vec![]))))
        .map(Json)
}

#[get("/<question_id>")]
fn get_question(question_id: i32, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    Question::get_full_question(question_id, &conn)
        .map(|group: (Question, User, Vec<Answer>)| QuestionResponse::from(QuestionData(group)))
        .map(Json)
}

#[post("/create", data = "<new_question>")]
fn create_question(new_question: Json<NewQuestionRequest>, _user: NormalUser, conn: Conn) -> Result<Json<QuestionResponse>, WeekendAtJoesError> {
    let request: NewQuestionRequest = new_question.into_inner();
    let user: User = User::get_by_id(request.author_id, &conn)?;
    Question::create_question(request.into(), &conn)
        .map(|question| QuestionData((question, user, vec![])))
        .map(QuestionResponse::from)
        .map(Json)
}



impl Routable for Question {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            get_questions_for_bucket,
            create_question,
            get_random_unanswered_question,
            get_questions_for_bucket,
            get_question,
        ]
    };
    const PATH: &'static str = "/question/";
}
