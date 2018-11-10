use crypto::scrypt;
use std::io;

/// Hashes the password to a String that should be stored in the database.
pub fn hash_password(password: &str) -> io::Result<String> {
    let params: scrypt::ScryptParams = scrypt::ScryptParams::new(10, 10, 10);
    scrypt::scrypt_simple(password, &params)
}

/// Checks the plaintext password against the hash string to see if the password is valid.
///
/// This can take upwards of 4 seconds to verify with the scrypt params present as of the time of
/// writing this comment.
pub fn verify_hash(password: &str, expected_hash: &str) -> Result<bool, &'static str> {
    scrypt::scrypt_check(password, expected_hash)
}
