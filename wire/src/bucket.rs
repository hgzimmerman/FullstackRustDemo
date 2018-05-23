use user::UserResponse;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BucketResponse {
    pub id: i32,
    pub bucket_name: String,
    pub is_public: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewBucketRequest {
    pub bucket_name: String,
    pub is_public: bool,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BucketUsersResponse {
    pub bucket: BucketResponse,
    pub users: Vec<UserResponse>
}