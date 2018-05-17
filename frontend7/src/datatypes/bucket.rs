use wire::bucket::*;
use util::input::InputState;


#[derive(Clone, Debug, PartialEq, Default)]
pub struct BucketData {
    pub id: i32,
    pub bucket_name: String
}

impl From<BucketResponse> for BucketData {
    fn from(response: BucketResponse) -> BucketData {
        BucketData {
            id: response.id,
            bucket_name: response.bucket_name
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NewBucket {
    pub name: InputState
}


impl NewBucket {
    pub fn validate_name(name: String) -> Result<String, String> {
        if name.len() < 1 {
            return Err("Bucket Name must have some text.".into())
        }
        Ok(name)
    }
    pub fn validate(&self) -> Result<NewBucketRequest, String> {
        Self::validate_name(self.name.inner_text())?;

        let request = NewBucketRequest {
            bucket_name: self.name.inner_text().clone(),
        };
        Ok(request)
    }
}
