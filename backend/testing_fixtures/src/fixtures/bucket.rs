use fixtures::user::UserFixture;
use db::{
    bucket::{
        Bucket,
        NewBucket,
        NewBucketUser
    },
    question::{
        Question,
        NewQuestion
    }
};


use diesel::PgConnection;
use chrono::{
    Utc,
    Duration
};
use Fixture;

const BUCKET_NAME_1: &'static str = "Private Bucket";
const BUCKET_NAME_2: &'static str = "Joinable Bucket";
const QUESTION_TEXT_1: &'static str = "Is this a question?";
const QUESTION_TEXT_2: &'static str = "Would your rather fight 100 horse sized horses or 1 duck sized duck?";


pub struct BucketFixture {
    pub user_fixture: UserFixture,
    pub private_bucket: Bucket,
    pub joinable_bucket: Bucket,
    pub question_1: Question,
    pub question_2: Question,
}


impl Fixture for BucketFixture {
    fn generate(conn: &PgConnection) -> Self {
        let user_fixture: UserFixture = UserFixture::generate(conn);

        // private bucket
        let new_bucket = NewBucket {
            bucket_name: BUCKET_NAME_1.to_string(),
            is_public_until: None,
        };
        let private_bucket = Bucket::create_bucket(new_bucket, conn).expect("create bucket");
        let owner = NewBucketUser {
            bucket_uuid: private_bucket.uuid,
            user_uuid: user_fixture.normal_user.uuid,
            owner: true,
            approved: true,
        };
        Bucket::add_user_to_bucket(owner,conn).expect("should add user to bucket");

        // Joinable bucket
        let new_bucket = NewBucket {
            bucket_name: BUCKET_NAME_2.to_string(),
            is_public_until: Some(Utc::now().naive_utc() + Duration::days(1)),
        };
        let joinable_bucket = Bucket::create_bucket(new_bucket, conn).expect("create bucket");

        let owner = NewBucketUser {
            bucket_uuid: joinable_bucket.uuid,
            user_uuid: user_fixture.normal_user.uuid,
            owner: true,
            approved: true,
        };
        Bucket::add_user_to_bucket(owner,conn).expect("should add user to bucket");

        let new_question = NewQuestion {
            bucket_uuid: private_bucket.uuid,
            author_uuid: user_fixture.normal_user.uuid,
            question_text: QUESTION_TEXT_1.to_string(),
            on_floor: false, // In the bucket
        };
        let question_1 = Question::create_question(new_question, conn).expect("Create question");


        let new_question = NewQuestion {
            bucket_uuid: private_bucket.uuid,
            author_uuid: user_fixture.normal_user.uuid,
            question_text: QUESTION_TEXT_2.to_string(),
            on_floor: false, // In the bucket
        };
        let question_2 = Question::create_question(new_question, conn).expect("Create question");

        BucketFixture {
            user_fixture,
            private_bucket,
            joinable_bucket,
            question_1,
            question_2,
        }
    }
}