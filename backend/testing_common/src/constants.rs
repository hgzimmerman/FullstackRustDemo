pub mod user {
    use auth_lib::hash_password;
    pub const PASSWORD: &'static str = "password";

    ///
    /// This constant is present in the common crate for compile-time related reasons.
    /// Because this crate rarely must be recompiled, keeping this long running function that produces
    /// a static value separate from typical test creation.
    lazy_static! {
        pub static ref PASSWORD_HASH: String = hash_password(PASSWORD).expect("Couldn't hash password.");
    }
}