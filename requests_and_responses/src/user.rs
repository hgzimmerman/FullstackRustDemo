//use chrono::NaiveDateTime;

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
    pub plaintext_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDisplayNameRequest {
    pub user_name: String,
    pub new_display_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRoleRequest {
    pub id: i32,
    pub user_role: i32,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwt {
    /// sub is the user id
    pub sub: i32,
    pub user_roles: Vec<UserRole>,
    /// exp is the Expiration date, in unix timestamp form
    pub exp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Unprivileged,
    Moderator,
    Admin,
    Publisher,
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> i32 {
        match role {
            UserRole::Unprivileged => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::Publisher => 4,
        }
    }
}

impl From<i32> for UserRole {
    fn from(number: i32) -> UserRole {
        match number {
            1 => UserRole::Unprivileged,
            2 => UserRole::Moderator,
            3 => UserRole::Admin,
            4 => UserRole::Publisher,
            _ => {
                eprintln!("Tried to convert an unsupported number into a user role");
                UserRole::Unprivileged
            }
        }
    }
}
