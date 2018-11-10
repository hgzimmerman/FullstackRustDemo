use rand::{
    self,
    Rng,
};

/// The secret contains a random string that is generated at startup.
/// This will be different every time the server restarts.
/// This secret randomization has the effect of invalidating JWTs whenever the server is restarted.
/// The Secret is used for creating and validating JWTs.
#[derive(Debug, Clone)]
pub struct Secret(pub String);

impl Secret {
    pub fn generate() -> Secret {
        let key = rand::thread_rng().gen_ascii_chars().take(256).collect::<String>();
        Secret(key)
    }

    pub fn from_user_supplied_string(key: &str) -> Secret {
        if key.len() <= 128 {
            panic!("The secret key must be equal to or greater than 128 characters.")
        } else if key.len() < 256 {
            warn!(
                "The secret key should be longer than 256 characters. It is {} characters long",
                key.len()
            );
        }
        Secret(key.to_string())
    }
}
