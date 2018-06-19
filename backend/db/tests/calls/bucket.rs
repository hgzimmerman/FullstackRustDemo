use db::bucket::{Bucket, NewBucket, NewBucketUser};
use db::question::{Question, NewQuestion};
use db::answer::{Answer, NewAnswer, AnswerData};
use db::user::User;

use common::setup::*;
use diesel::PgConnection;
use chrono::Utc;
use chrono::Duration;
use identifiers::user::UserUuid;
use identifiers::bucket::BucketUuid;
use identifiers::question::QuestionUuid;

use db::CreatableUuid;
use testing_fixtures::fixtures::bucket::BucketFixture;



#[test]
fn get_public_buckets() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.admin_user.uuid);
        let buckets = Bucket::get_public_buckets(user_uuid, conn).expect("get public buckets");
        assert_eq!(buckets.len(), 1); // there should be 1 bucket.
    });
}
#[test]
fn get_public_buckets_expired() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.admin_user.uuid);
        // A bucket with a is_public_until value in the past should not count as public.
        let new_bucket = NewBucket {
            bucket_name: "Old Bucket".to_string(),
            is_public_until: Some(Utc::now().naive_utc() - Duration::days(1)), // A bucket that is no longer public due to expiring.
        };
        Bucket::create(new_bucket, conn).expect("create bucket");
        let buckets = Bucket::get_public_buckets(user_uuid, conn).expect("get public buckets");
        assert_eq!(buckets.len(), 1); // there should be 1 bucket.

    });
}
#[test]
fn get_public_buckets_dont_show_already_joined() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let buckets = Bucket::get_public_buckets(user_uuid, conn).expect("get public buckets");
        assert_eq!(buckets.len(), 0); // there should be 0 bucket because the normal user already owns every bucket.
    });
}
#[test]
fn set_publicity() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let bucket_uuid: BucketUuid = BucketUuid(fixture.private_bucket.uuid);
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.admin_user.uuid);
        Bucket::set_bucket_publicity(bucket_uuid, true, conn).expect("set publicity");
        let buckets = Bucket::get_public_buckets(user_uuid, conn).expect("get public buckets");
        assert_eq!(buckets.len(), 2); // Both buckets should be public now.
    });
}

#[test]
fn get_users_buckets() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let buckets = Bucket::get_buckets_user_belongs_to(user_uuid, conn).expect("get public buckets");
        assert_eq!(buckets.len(), 2);

        // The admin user doesn't belong to any buckets
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.admin_user.uuid);
        let buckets = Bucket::get_buckets_user_belongs_to(user_uuid, conn).expect("get public buckets");
        assert_eq!(buckets.len(), 0);
    });
}

#[test]
fn is_user_bucket_owner() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let bucket_uuid: BucketUuid = BucketUuid(fixture.joinable_bucket.uuid);
        let is_owner: bool = Bucket::is_user_owner(user_uuid, bucket_uuid, conn);
        assert!(is_owner);


        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.admin_user.uuid);
        let is_owner: bool = Bucket::is_user_owner(user_uuid, bucket_uuid, conn);
        assert!(!is_owner);
    });
}

#[test]
fn is_user_allowed_to_join_bucket() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let bucket_uuid: BucketUuid = BucketUuid(fixture.joinable_bucket.uuid);
        let is_approved: bool = Bucket::is_user_approved(user_uuid, bucket_uuid, conn);
        assert!(is_approved);


        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.admin_user.uuid);
        let is_approved: bool = Bucket::is_user_approved(user_uuid, bucket_uuid, conn);
        assert!(!is_approved);
    });
}

#[test]
fn approval_empty() {

    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let user_uuid: UserUuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let bucket_uuid: BucketUuid = BucketUuid(fixture.joinable_bucket.uuid);

        let users: Vec<User> = Bucket::get_users_with_approval(bucket_uuid, user_uuid, conn).expect("get_users_with_approval");
        assert_eq!(users.len(), 0)
    });
}


#[test]
fn approval_join_request() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let owner_user_uuid: UserUuid = UserUuid(fixture.user_fixture.normal_user.uuid);
        let bucket_uuid: BucketUuid = BucketUuid(fixture.joinable_bucket.uuid);
        let join_user_uuid: UserUuid = UserUuid(fixture.user_fixture.admin_user.uuid);

        // Request to join the bucket.
        let join_request_user = NewBucketUser {
            bucket_uuid: bucket_uuid.0,
            user_uuid: join_user_uuid.0,
            owner: false,
            approved: false,
        };
        Bucket::add_user_to_bucket(join_request_user, conn).expect("add user to bucket as a join request");

        let users: Vec<User> = Bucket::get_users_with_approval(bucket_uuid, owner_user_uuid, conn).expect("get_users_with_approval");
        assert_eq!(users.len(), 0);

        // Approve the user.
        Bucket::set_user_approval(join_user_uuid, bucket_uuid, true, conn).expect("set approval");

        // Check that it is approved
        let users: Vec<User> = Bucket::get_users_with_approval(bucket_uuid, owner_user_uuid, conn).expect("get_users_with_approval");
        assert_eq!(users.len(), 1);
        let is_approved: bool = Bucket::is_user_approved(join_user_uuid, bucket_uuid, conn);
        assert!(is_approved)
    });
}

/// Even though the admin user should not be able to add a question
/// because it doesn't belong to the bucket,
/// the DB method is not responsible for enforcing that constraint.
#[test]
fn add_question_to_bucket() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let new_question = NewQuestion {
            bucket_uuid: fixture.private_bucket.uuid,
            author_uuid: fixture.user_fixture.admin_user.uuid, // Admin user does not belong to the bucket
            question_text: "new question".to_string(),
            on_floor: false, // In the bucket
        };
        Question::create_data(new_question, conn)
            .expect("Should create question.");
    });
}

#[test]
fn floor_bucket_location() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let question_uuid_1: QuestionUuid = QuestionUuid(fixture.question_1.uuid);
        let question_uuid_2: QuestionUuid = QuestionUuid(fixture.question_2.uuid);
        let bucket_uuid: BucketUuid = BucketUuid(fixture.private_bucket.uuid);
        Question::put_question_on_floor(question_uuid_1, conn).expect("should put the question on the floor");
        Question::put_question_on_floor(question_uuid_2, conn).expect("should put the question on the floor");
        Question::get_random_question(bucket_uuid, conn).expect_err("There should be no questions available to draw.");
        let count = Question::get_number_of_questions_in_bucket(bucket_uuid, conn).expect("should get number of questions in bucket");
        assert_eq!(count, 0);

        Question::put_question_in_bucket(question_uuid_1, conn).expect("should put the question back in bucket");
        let questiondata = Question::get_random_question(bucket_uuid, conn).expect("There should be one question available to draw.");
        assert_eq!(questiondata.question.uuid, question_uuid_1.0);
        let count = Question::get_number_of_questions_in_bucket(bucket_uuid, conn).expect("should get number of questions in bucket");
        assert_eq!(count, 1);
    });
}

#[test]
fn delete_question() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let question_uuid_1: QuestionUuid = QuestionUuid(fixture.question_1.uuid);
        let question_uuid_2: QuestionUuid = QuestionUuid(fixture.question_2.uuid);
        let bucket_uuid: BucketUuid = BucketUuid(fixture.private_bucket.uuid);
        Question::delete_question(question_uuid_1, conn).expect("should put the question on the floor");
        Question::delete_question(question_uuid_2, conn).expect("should put the question on the floor");
        Question::get_random_question(bucket_uuid, conn).expect_err("There should be no questions available to draw.");
        let count = Question::get_number_of_questions_in_bucket(bucket_uuid, conn).expect("should get number of questions in bucket");
        assert_eq!(count, 0);
    });
}

#[test]
fn answer_question() {
    setup(|fixture: &BucketFixture, conn: &PgConnection| {
        let question_uuid: QuestionUuid = QuestionUuid(fixture.question_1.uuid);
        let answer_text = Some("no".to_string());
        let new_answer = NewAnswer {
            author_uuid: fixture.user_fixture.normal_user.uuid,
            question_uuid: question_uuid.0,
            answer_text: answer_text.clone(),
        };
        Answer::create(new_answer, conn).expect("Should create answer");
        let question_data = Question::get_full_question(question_uuid, conn).expect("should get full question");
        assert!(question_data.answers.len() > 0);
        let answer_data: &AnswerData = question_data.answers.first().expect("Should be 1 answer");
        assert_eq!(answer_data.answer.answer_text, answer_text);
    });
}