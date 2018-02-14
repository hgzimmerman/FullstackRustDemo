    use rocket::Route;
    use rocket_contrib::Json;

    use routes::Routable;
    use db::Conn;

    use db::user::User;
    use requests_and_responses::user::{NewUserRequest, UpdateDisplayNameRequest, UserResponse};

    use error::WeekendAtJoesError;
    use auth::user_authorization::*;



    #[get("/<user_id>")]
    fn get_user(user_id: i32, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
        User::get_user(user_id, &conn).and_then(|user|{
            let user_response: UserResponse = user.into();
            Ok(Json(user_response))
        })
    }

    #[get("/users/<num_users>")]
    fn get_users(num_users: i64, conn: Conn) -> Result<Json<Vec<UserResponse>>, WeekendAtJoesError> {
        User::get_users(num_users, &conn).and_then(|users|{
            let user_responses: Vec<UserResponse> = users.into_iter().map(|user| user.into()).collect();
            Ok(Json(user_responses))
        })
    }



    #[post("/", data = "<new_user>")]
    pub fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
        let new_user: NewUserRequest = new_user.into_inner();
        User::create_user(new_user, &conn).and_then(|user| {
            let user_response: UserResponse = user.into();
            Ok(Json(user_response))
        })
    }


    #[put("/", data = "<data>")]
    fn update_user_display_name(data: Json<UpdateDisplayNameRequest>, _user: NormalUser, conn: Conn ) -> Option<Json<UserResponse>> {
        info!("updating user display name");
        let request: UpdateDisplayNameRequest = data.into_inner();
        let updated_user = User::update_user_display_name(request, &conn);

        match updated_user {
            Ok(updated_user) => {
                let user_response: UserResponse = updated_user.into();
                Some(Json(user_response))
            }
            Err(_) => None
        }
    }


    #[delete("/<user_id>")]
    fn delete_user(user_id: i32, _admin: AdminUser, conn: Conn) -> Option<Json<UserResponse>> {

        let updated_user = User::delete_user_by_id(user_id, &conn);

        match updated_user {
            Ok(updated_user) => {
                let user_response: UserResponse = updated_user.into();
                Some(Json(user_response))
            }
            Err(_) => None
        }
    }

    #[delete("/<user_name>", rank = 2)]
    pub fn delete_user_by_name(user_name: String, _admin: AdminUser,conn: Conn) -> Option<Json<UserResponse>> {

        let updated_user = User::delete_user_by_name(user_name, &conn);

        match updated_user {
            Ok(updated_user) => {
                let user_response: UserResponse = updated_user.into();
                Some(Json(user_response))
            }
            Err(e) => {
                info!("Couldn't delete user. Reason: {}", e);
                None
            }
        }
    }

    // Export the ROUTES and their path
    impl Routable for User {
        const ROUTES: &'static Fn() -> Vec<Route> = &|| routes!
        [
            create_user,
            update_user_display_name,
            get_user,
            get_users,
            delete_user,
            delete_user_by_name
        ];
        const PATH: &'static str = "/user/";
    }


