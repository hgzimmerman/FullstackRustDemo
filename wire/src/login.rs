use user::*;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub jwt: String,
    pub user: UserResponse,
}
