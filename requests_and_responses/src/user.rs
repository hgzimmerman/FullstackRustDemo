/// User to be sent over the wire
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub user_name: String,
    pub display_name: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullUserResponse {
    pub user_name: String,
    pub display_name: String,
    pub id: i32,
    pub locked: bool,
    pub banned: bool,
    // pub roles: UserRoleResponse
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserRequest {
    pub user_name: String,
    pub display_name: String,
    pub plaintext_password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDisplayNameRequest {
    pub user_name: String,
    pub new_display_name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRoleRequest {
    pub id: i32,
    pub user_role: i32
}
