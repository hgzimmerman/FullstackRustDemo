use answer::*;
use wire::answer::*;
use identifiers::answer::AnswerUuid;
use identifiers::user::UserUuid;

impl From<AnswerData> for AnswerResponse {
    fn from(data: AnswerData) -> AnswerResponse {
        AnswerResponse {
            uuid: AnswerUuid(data.answer.uuid),
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
    pub fn attach_user_id(request: NewAnswerRequest, user_uuid: UserUuid) -> NewAnswer {
        NewAnswer {
            answer_text: request.answer_text,
            author_uuid: user_uuid.0,
            question_uuid: request.question_uuid.0,
        }
    }
}
