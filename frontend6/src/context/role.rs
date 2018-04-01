use super::Context;


impl Context {

    fn is_user_normal_user(&mut self) -> bool {
        self.restore_jwt();
        true
    }

    fn is_user_moderator(&mut self) -> bool {

        self.restore_jwt();
        true
    }

    fn is_user_admin(&mut self) -> bool {

        self.restore_jwt();
        true
    }

    fn is_user_publisher(&mut self) -> bool {
        true
    }

}