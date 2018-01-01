use views::loadable::Loadable;
use controller::{Msg, Updatable};
use yew::html::Context;
use yew::services::format::{Nothing, Json};
use yew::services::fetch::{FetchService, Method};

use yew::services::format::Storable;


use models::BucketModel;
use models::Question;


pub enum BucketMsg {
    /// Send the question to the server, update the view to reset the textbox for the new question.
    AddQuestion,
    /// Set the model's new question.
    BuildQuestion(String),
    /// Skip the question, putting it back into the bucket.
    SkipQuestion,
    /// Answer the question, removing it from the bucket, and putting it on the floor.
    AnswerQuestion,
    /// Set the model's answer.
    BuildAnswer(String),
    /// Request a new active question from the bucket via the server.
    DrawQuestion,
    /// Set the model's active question
    SetActiveQuestion(Question),

//    WsConnect  // -> HandleWsUpdate
    //HandleWSUpdate // update the model/ views based on what was given via the ws connection handle
//    WsDisconnect
    //WsLost
}

impl Updatable<BucketMsg> for BucketModel {
    fn update(&mut self, context: &mut Context<Msg>, msg: BucketMsg) {
        use self::BucketMsg::*;
        match msg {
            AddQuestion => {

                let new_question: Question = Question {
                    question: self.new_question_input.clone(),
                    answer: None,
                    author: "Joe".to_string(),
                    answered_by: None,
                    id: 0
                };

                let route = format!("/api/bucket/{session}/create", session=self.session_id);


//                use controller::news::NewsMsg::ArticleReady;
                context.fetch(Method::Post,
                              route.as_str(),
//                              Storable::from(Json(new_question)),
                              Nothing, // Todo, ^^^ can't do this for some reason
                              |n| {
                                  let n: Nothing = n; // establish what n is - in order to make the compiler happy
                                  Msg::NoOp
                              }
                );

                self.new_question_input = String::from("");
            }
            BuildQuestion(question_text) => {
                self.new_question_input = question_text;
            }
            SkipQuestion => {
                // TODO send request to put question w/ active_question.id back in the bucket, move active user to next.

                self.active_question = None;
            }
            AnswerQuestion => {
                // TODO send request to put question w/id on floor, move active user to next.

                self.active_question = None;
            }
            BuildAnswer(answer_text) => {
                self.answer_input = answer_text;
            }
            DrawQuestion => {
                // TODO make request draw question. Include username in request.
                let temporary = Question {
                    question: "How are you doing?".to_string(),
                    answer: None,
                    author: "Me".to_string(),
                    answered_by: None,
                    id: 0,
                };
                self.active_question = Some(temporary) // Todo -> move this to SetActiveQuestion via the fetch service.
            }
            SetActiveQuestion(new_active_question) => {
                self.active_question = Some(new_active_question)
            }
        }
    }
}