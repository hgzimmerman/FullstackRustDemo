use common::fetch::FetchRequest;
use common::fetch::Auth;
use common::fetch::HttpMethod;
use common::fetch::to_body;

use identifiers::bucket::BucketUuid;
use identifiers::user::UserUuid;
use identifiers::question::QuestionUuid;
use wire::bucket::*;
use wire::question::*;
use wire::answer::*;


#[derive(Serialize, Deserialize)]
pub enum BucketRequest {
    GetPublicBuckets,
    GetBucketsForUser,
    GetBucket{bucket_uuid: BucketUuid},
    CreateBucket(NewBucketRequest),
    GetRandomQuestion { bucket_uuid: BucketUuid },
    GetQuestions { bucket_uuid: BucketUuid},
    AnswerQuestion(NewAnswerRequest),
    CreateQuestion(NewQuestionRequest),
    DeleteQuestion{question_uuid: QuestionUuid},
    PutQuestionBackInBucket{question_uuid: QuestionUuid},
    SetBucketPublicStatus{bucket_uuid: BucketUuid, is_public: bool},
    ApproveUserForBucket {bucket_uuid: BucketUuid, user_uuid: UserUuid},
    RemoveUserFromBucket {bucket_uuid: BucketUuid, user_uuid: UserUuid},
    GetUnapprovedUsersForOwnedBuckets,
    GetUsersInBucket{bucket_uuid: BucketUuid},
    GetIsUserOwnerOfBucket{bucket_uuid: BucketUuid},
    CreateJoinBucketRequest {bucket_uuid: BucketUuid},
    GetNumberOfQuestionsInBucket {bucket_uuid: BucketUuid}
}

impl FetchRequest for BucketRequest {
    fn resolve_path(&self) -> String {
        use self::BucketRequest::*;
        match *self {
            GetPublicBuckets => "buckets/public".into(),
            GetBucketsForUser => "buckets/approved".into(),
            GetBucket{bucket_uuid} => format!("buckets/{}", bucket_uuid),
            CreateBucket(_) => "buckets/create".into(),
            GetRandomQuestion { bucket_uuid } => format!("question/random_question?bucket_uuid={}", bucket_uuid),
            GetQuestions { bucket_uuid } => format!("question?bucket_uuid={}", bucket_uuid),
            AnswerQuestion(_) => "answer/create".into(),
            CreateQuestion(_) => "question/create".into(),
            DeleteQuestion {question_uuid} => format!("question/{}", question_uuid),
            PutQuestionBackInBucket {question_uuid} => format!("question/{}/into_bucket", question_uuid),
            SetBucketPublicStatus {bucket_uuid, is_public} => format!("buckets/{}/publicity?is_public={}", bucket_uuid, is_public),
            ApproveUserForBucket {bucket_uuid, user_uuid} => format!("buckets/{}/approval?user_id={}",bucket_uuid, user_uuid),
            RemoveUserFromBucket {bucket_uuid, user_uuid} => format!("buckets/{}?user_id={}",bucket_uuid, user_uuid),
            GetUnapprovedUsersForOwnedBuckets => "buckets/unapproved_users_for_owned_buckets".into(),
            GetUsersInBucket {bucket_uuid} => format!("buckets/{}/users", bucket_uuid),
            GetIsUserOwnerOfBucket {bucket_uuid}  => format!{"buckets/{}/user_owner_status", bucket_uuid},
            CreateJoinBucketRequest {bucket_uuid} => format!{"buckets/{}/user_join_request", bucket_uuid},
            GetNumberOfQuestionsInBucket {bucket_uuid} => format!("/api/question/quantity_in_bucket?bucket_uuid={}", bucket_uuid)
        }
    }
    fn resolve_auth(&self) -> Auth {
        use self::BucketRequest::*;
        use self::Auth::*;
        match *self {
            GetPublicBuckets => Required,
            GetBucketsForUser => Required,
            GetBucket{..} => Required,
            CreateBucket(_) => Required,
            GetRandomQuestion {..} => NotRequired,
            GetQuestions {..} => NotRequired,
            AnswerQuestion(_) => Required,
            CreateQuestion(_) => Required,
            DeleteQuestion {..} => Required,
            PutQuestionBackInBucket {..} => Required,
            SetBucketPublicStatus {..} => Required,
            ApproveUserForBucket {..} => Required,
            RemoveUserFromBucket {..} => Required,
            GetUnapprovedUsersForOwnedBuckets => Required,
            GetUsersInBucket {..} => Required,
            GetIsUserOwnerOfBucket {..} => Required,
            CreateJoinBucketRequest {..} => Required,
            GetNumberOfQuestionsInBucket {..} => Required
        }
    }
    fn resolve_body_and_method(&self) -> HttpMethod {
        use self::BucketRequest::*;
        use self::HttpMethod::*;

        let empty: String = "".to_string();
        match self {
            GetPublicBuckets => Get,
            GetBucketsForUser => Get,
            GetBucket {..} => Get,
            CreateBucket(r) => Post(to_body(r)),
            GetRandomQuestion {..} => Get,
            GetQuestions {..} => Get,
            AnswerQuestion(r) => Post(to_body(r)),
            CreateQuestion(r) => Post(to_body(r)),
            DeleteQuestion {..} => Delete,
            PutQuestionBackInBucket {..} => Put(empty), // no body
            SetBucketPublicStatus {..} => Put(empty),
            ApproveUserForBucket {..} => Put(empty),
            RemoveUserFromBucket {..} => Delete,
            GetUnapprovedUsersForOwnedBuckets => Get,
            GetUsersInBucket {..} => Get,
            GetIsUserOwnerOfBucket {..} => Get,
            CreateJoinBucketRequest {..} => Post(empty),
            GetNumberOfQuestionsInBucket {..} => Get
        }
    }
}
