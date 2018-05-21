
use super::Context;
use failure::Error;




impl Context {
    pub fn store_jwt(&mut self, jwt: String) {
        self.local_storage.store("JWT", jwt)
    }

    pub fn restore_jwt(&mut self) -> Result<String, Error> {
        self.local_storage.restore("JWT")
    }

    pub fn is_logged_in(&mut self) -> bool {
        self.restore_jwt().is_ok()
    }

    /// Functionally logs the user out
    pub fn remove_jwt(&mut self) {
        self.local_storage.remove("JWT");
    }
}
