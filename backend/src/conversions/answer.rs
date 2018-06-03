use db::answer::*;
use wire::answer::*;
use identifiers::answer::AnswerUuid;

impl From<AnswerData> for AnswerResponse {
    fn from(data: AnswerData) -> AnswerResponse {
        AnswerResponse {
            id: AnswerUuid(data.answer.id),
            answer_text: data.answer.answer_text,
            author: data.user.into(),
        }
    }
}

//impl From<NewAnswerRequest> for NewAnswer {
//    fn from(request: NewAnswerRequest) -> NewAnswer {
//        NewAnswer {
//            answer_text: request.answer_text,
//            author_id: request.author_id,
//            question_id: request.question_id,
//        }
//    }
//}
impl NewAnswer {
    pub fn attach_user_id(request: NewAnswerRequest, user_id: i32) -> NewAnswer {
        NewAnswer {
            answer_text: request.answer_text,
            author_id: user_id,
            question_id: request.question_id.0,
        }
    }
}
