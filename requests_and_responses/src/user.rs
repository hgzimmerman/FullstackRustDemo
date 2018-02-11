/// User to be sent over the wire
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub user_name: String,
    pub display_name: String,
    pub id: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserRequest {
    pub user_name: String,
    pub display_name: String,
    pub plaintext_password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDisplayNameRequest {
    pub id: i32,
    pub new_display_name: String
}

