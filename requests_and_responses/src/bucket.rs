#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BucketResponse {
    pub id: i32,
    pub bucket_name: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NewBucketRequest {
    pub bucket_name: String,
}

