// use rocket::Route;
// use rocket_contrib::Json;
// use super::Routable;
// use std::sync::Mutex;
// use std::collections::HashMap;
// use rand::{thread_rng, Rng};
// use rocket::State;

// pub struct BucketSessions(pub HashMap<String, Bucket>);

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Bucket {
//     active_question: Option<Question>,
//     active_user: String, // Make user
//     connected_users: Vec<String>, // Todo, make this hold websocket instances??
//     bucket: Vec<Question>,
//     floor: Vec<Question>
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Question {
//     question: String,
//     answer: Option<String>,
//     author: String,
//     answered_by: Option<String>,
//     id: usize
// }


// impl Bucket {
//     pub fn new() -> Bucket {
//         Bucket {
//             active_question: None,
//             active_user: "Joe".to_string(),
//             connected_users: Vec::new(),
//             bucket: Vec::new(),
//             floor: Vec::new(),
//         }
//     }
// }


// #[post("/<session_id>/create", data = "<new_question>", rank=0)]
// fn create_question(new_question: Json<Question>, session_id: String, sessions: State<Mutex<BucketSessions>>) -> Json<Question> {
//     let mut sessions = sessions.lock().unwrap();
//     let bucket: &mut Bucket = sessions.0.get_mut(&session_id).unwrap();
//     let new_question: Question = new_question.into_inner();
//     bucket.bucket.push(new_question.clone());

//     Json(new_question)
// }

// #[get("/<session_id>/draw", rank=0)]
// fn draw_question(session_id: String, sessions: State<Mutex<BucketSessions>>) -> Json<Question> {
//     let mut sessions = sessions.lock().unwrap();
//     let bucket: &mut Bucket = sessions.0.get_mut(&session_id).unwrap();
//     let mut rng = thread_rng();

//     let chosen_question = rng.choose(&bucket.bucket).cloned();
//     bucket.active_question = chosen_question.clone();
//     // TODO, remove the question from the bucket

//     Json(chosen_question.unwrap())
// }

// #[post("/<session_id>/answer", data = "<answered_question>", rank=0)]
// fn answer_question(answered_question: Json<Question>, session_id: String, sessions: State<Mutex<BucketSessions>>) -> Json<Question> {
//     let mut sessions = sessions.lock().unwrap();
//     let bucket: &mut Bucket = sessions.0.get_mut(&session_id).unwrap();

//     let answered_question: Question = answered_question.into_inner();

//     bucket.floor.push(answered_question.clone());

//     Json(answered_question)
// }


// pub fn bucket_routes() -> Vec<Route> {
//     routes![create_question, draw_question, answer_question]
// }


// impl Routable for Bucket {
//     const ROUTES: &'static Fn() -> Vec<Route> = &||bucket_routes();
//     const PATH: &'static str = "/bucket/";
// }
