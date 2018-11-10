use identifiers::user::UserUuid;
use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        RwLock,
    },
};
use warp::{
    filters::BoxedFilter,
    Filter,
};

/// The BannedList contains a list of users that should not have access to the system.
/// This is an ephemeral protection mechanism, that will prevent users from using outstanding JWTs if their UUID is in this structure.
/// If the server is restarted, and the JWT key is not rotated, then this will offer no protection
/// as there is no persistence aspect to this.
#[derive(Debug, Default, Clone)]
pub struct BannedList(Arc<RwLock<BTreeSet<UserUuid>>>);

impl BannedList {
    /// Bans the user from this server.
    pub fn ban(&self, user: UserUuid) {
        self.0.write().unwrap().insert(user);
    }

    /// Unbans the user from this server.
    pub fn unban(&self, user: &UserUuid) {
        self.0.write().unwrap().remove(user);
    }

    /// Checks if a given user is banned.
    pub fn is_banned(&self, user: &UserUuid) -> bool {
        self.0.read().unwrap().contains(user)
    }
}

pub fn banned_list_filter(banned_list: BannedList) -> BoxedFilter<(BannedList,)> {
    warp::any().map(move || banned_list.clone()).boxed()
}
