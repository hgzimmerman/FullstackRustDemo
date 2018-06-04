use std::collections::HashSet;
use std::sync::RwLock;

use identifiers::user::UserUuid;

/// Is a Set that holds all of the users that have been banned since the last time the server was
/// restarted.
///
/// This set is used to prevent banned users JWTs from authenticating requests before they expire.
/// Once a user is banned, JWTs won't be issued for that account, but without this stateful set,
/// they could do anything until the JWT expires.
///
/// By checking this set for every authenticated request, we can enforce a ban without having to
/// make a request to the database to check for a banned flag.
pub struct BannedSet(RwLock<HashSet<UserUuid>>);

impl BannedSet {
    pub fn new() -> BannedSet {
        BannedSet(RwLock::new(HashSet::new()))
    }

    pub fn ban_user(&self, user_id: UserUuid) {
        self.0.write().unwrap().insert(user_id);
    }
    /// True indicates that the user was unbanned.
    /// False indicates that the user was not in the banned set to begin with.
    pub fn unban_user(&self, user_id: &UserUuid) -> bool {
        self.0.write().unwrap().remove(user_id)
    }

    pub fn is_user_banned(&self, user_id: &UserUuid) -> bool {
        self.0.read().unwrap().contains(user_id)
    }
}
