// TODO move these to a util file
use warp::Reply;
use serde::Serialize;
use wire::convert_vector;

use warp::filters::BoxedFilter;
use serde::Deserialize;
use uuid::Uuid;
use std::collections::HashMap;
use warp::Filter;

/// Util function that makes replying easier
pub fn convert_and_json<T, U>(source: T) -> impl Reply where
    T: Into<U>,
    U: Serialize
{
    let target: U = source.into();
    warp::reply::json(&target)
}


/// Converts a vector of T to a vector of U then converts the U vector to a JSON reply.
pub fn convert_vector_and_json<T, U>(source: Vec<T>) -> impl Reply where
    U: From<T>,
    U: Serialize
{
    let target: Vec<U> = convert_vector(source);
    warp::reply::json(&target)
}



pub fn json_body_filter<T> (kb_limit: u64) -> BoxedFilter<(T,)>
    where
        T: for<'de> Deserialize<'de> + Send + Sync + 'static
{
        warp::body::content_length_limit(1024 * kb_limit)
            .and(warp::body::json())
            .boxed()
}

pub fn query_uuid(key: &'static str) -> BoxedFilter<(Uuid,)> {
    warp::query::query::<HashMap<String, String>>()
        .and_then(move |hm: HashMap<String,String>| {
            hm.get(key)
                .and_then(|value: &String| {
                    Uuid::parse_str(&value).ok()
                })
                .ok_or(warp::reject())
        })
        .boxed()
}


#[cfg(test)]
pub mod test {
    use bytes::Bytes;
    use std::ops::Deref;
    use serde_json::from_str;
    use warp::http::Response;
    use serde::Deserialize;

    /// Used in testing, this function will try to deserialize a response generated from a typical
    /// warp::testing::request() invocation.
    pub fn deserialize<T: for<'de> Deserialize<'de>>(response: Response<Bytes>) -> T {
        let body = response.into_body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        println!("Body string: {}", body_string);
        from_str::<T>(body_string).expect("Should be able to deserialize body")
    }

    pub fn deserialize_string(response: Response<Bytes>) -> String {
        let body = response.into_body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        String::from(body_string)
    }

}