use common::fetch::FetchRequest;
use common::fetch::Auth;
use common::fetch::HttpMethod;
use common::fetch::to_body;

use wire::user::NewUserRequest;
use wire::login::LoginRequest;

#[derive(Serialize, Deserialize)]
pub enum AuthRequest {
    Login(LoginRequest),
    CreateUser(NewUserRequest),
}

impl FetchRequest for AuthRequest {
    fn resolve_path(&self) -> String {
        use self::AuthRequest::*;
        match *self {
            Login(_) => "auth/login".into(),
            CreateUser(_) => "user/".into(),
        }
    }
    fn resolve_auth(&self) -> Auth {
        Auth::NotRequired
    }
    fn resolve_body_and_method(&self) -> HttpMethod {
        use self::AuthRequest::*;
        use self::HttpMethod::*;
        match self {
            Login(r) => Post(to_body(r)),
            CreateUser(r) => Post(to_body(r)),
        }
    }
}
