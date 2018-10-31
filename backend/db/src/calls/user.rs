use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use chrono::{NaiveDateTime, Utc, Duration};
use crate::schema::users;
use error::JoeResult;
use diesel::PgConnection;
use identifiers::user::UserUuid;
use uuid::Uuid;

//use log::info;
use log::info;

// TODO, I don't think that this file should have wire types
use wire::user::*;


/// The database's representation of a user.
#[derive(Debug, Clone, Identifiable, Queryable, CrdUuid, ErrorHandler, PartialEq, TypeName)]
#[primary_key(uuid)]
#[insertable = "NewUser"]
#[table_name = "users"]
pub struct User {
    /// The primary key
    pub uuid: Uuid,
    /// The user name of the user. This is used primarily for logging in, and is seldom displayed.
    pub user_name: String,
    /// This name will be displayed on data associated with the user, such as forum posts, or as the author of articles.
    pub display_name: String,
    /// The stored hash of the password.
    pub password_hash: String,
    /// If the user is locked, they cannot try to log in until the timer expires.
    /// If the user fails a password attempt, lock them out for n seconds.
    pub locked: Option<NaiveDateTime>,
    pub failed_login_count: i32,
    /// If the user is banned, they cannot log in or access routes that require JWT tokens.
    pub banned: bool,
    /// The roles of the user.
    pub roles: Vec<i32>, // currently this is stored as an int. It would be better to store it as an enum, if diesel-enum serialization can be made to work.
}


#[derive(Insertable, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub display_name: String,
    pub password_hash: String,
    pub failed_login_count: i32,
    pub banned: bool,
    pub roles: Vec<i32>,
    // pub locked: Option<NaiveDateTime>,
}


impl User {
    /// Gets the user by their user name.
    pub fn get_user_by_user_name(name: &str, conn: &PgConnection) -> JoeResult<User> {
        use crate::schema::users::dsl::*;
        info!("Getting user with Name: {}", name);

        users
            .filter(user_name.eq(name))
            .first::<User>(conn)
            .map_err(User::handle_error)
    }



    /// Gets a vector of users of length n.
    // TODO: consider also specifing a step, so that this can be used in a proper pagenation system.
    pub fn get_users(num_users: i64, conn: &PgConnection) -> JoeResult<Vec<User>> {
        use crate::schema::users::dsl::*;
        users
            .limit(num_users)
            .load::<User>(conn)
            .map_err(User::handle_error)
    }

    // TODO make this take a list of roles.
    /// For the given role, get all users with the that role.
    pub fn get_users_with_role(user_role: UserRole, conn: &PgConnection) -> JoeResult<Vec<User>> {

        let user_role_id: i32 = i32::from(user_role);

        use crate::schema::users::dsl::*;
        use crate::schema::users;
        use diesel::PgArrayExpressionMethods;

        // Diesel can construct queries that operate on the contents of Postgres arrays.
        users
            .filter(users::roles.contains(
                vec![user_role_id],
            ))
            .load::<User>(conn)
            .map_err(User::handle_error)
    }

    /// If the user has their banned flag set, this will return true.
    pub fn is_user_banned(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<bool> {
        use crate::schema::users::dsl::*;

        users
            .find(user_uuid.0)
            .select(banned)
            .first::<bool>(conn)
            .map_err(User::handle_error)
    }

    // TODO, refactor this, only implement the db transaction, logic can go in the login method
    pub fn check_if_locked(&self, conn: &PgConnection) -> JoeResult<bool> {
        use crate::schema::users::dsl::*;
        use crate::schema::users;

        if let Some(l) = self.locked {
            let current_date = Utc::now().naive_utc();
            if current_date > l {
                Ok(true)
            } else {
                // Remove the locked status
                let target = users.filter(users::uuid.eq(self.uuid));
                diesel::update(target)
                    .set(locked.eq(None::<NaiveDateTime>))
                    .execute(conn)
                    .map_err(User::handle_error)?;
                Ok(false)
            }
        } else {
            // No need to remove a lock status that isn't present.
            Ok(false)
        }
    }

    /// Resets the login failure count to 0.
    /// This should be called after the user logs in successfully.
    pub fn reset_login_failure_count(user_uuid: UserUuid, conn: &PgConnection) -> JoeResult<()> {
        use crate::schema::users::dsl::*;
        use crate::schema::users;

        let target = users.filter(
            users::uuid.eq(user_uuid.0),
        );
        diesel::update(target)
            .set(failed_login_count.eq(0))
            .execute(conn)
            .map_err(User::handle_error)?;
        Ok(())
    }

    /// This method is to be called after a user has failed to log in.
    /// Based on the number of current failed login attempts in a row, it will calculate the locked period.
    /// It will then store the datetime of unlock, along with an incremented failure count, so that next time it will take longer.
    pub fn record_failed_login(user_uuid: UserUuid, current_failed_attempts: i32, conn: &PgConnection) -> JoeResult<NaiveDateTime> {
        use crate::schema::users::dsl::*;
        use crate::schema::users;

        info!("record_failed_login: setting the expire time and failure count");
        let current_date = Utc::now().naive_utc();
        let delay_seconds: i64 = (current_failed_attempts * 2).into(); // Todo: come up with a better function than this
        let expire_datetime = current_date + Duration::seconds(delay_seconds);
        let new_failed_attempts = current_failed_attempts + 1; // Increment the failed count

        let target = users.filter(
            users::uuid.eq(user_uuid.0),
        );
        let _ = diesel::update(target)
            .set((
                locked.eq(expire_datetime),
                failed_login_count.eq(
                    new_failed_attempts
                ),
            ))
            .execute(conn)
            .map_err(User::handle_error)?;

        return Ok(expire_datetime);
    }

    /// Banns or unbans the user.
    pub fn set_ban_status(user_uuid: UserUuid, is_banned: bool, conn: &PgConnection) -> JoeResult<User> {
        use crate::schema::users::dsl::*;
        use crate::schema::users;
        let target = users.filter(
            users::uuid.eq(user_uuid.0),
        );
        diesel::update(target)
            .set(banned.eq(is_banned))
            .get_result(conn)
            .map_err(User::handle_error)

    }

    /// Adds a role to the user.
    pub fn add_role_to_user(user_uuid: UserUuid, user_role: UserRole, conn: &PgConnection) -> JoeResult<User> {

        use crate::schema::users::dsl::*;
        use crate::schema::users;

        let user = User::get_by_uuid(user_uuid.0, conn)?;

        let user_role_id: i32 = i32::from(user_role);
        if user.roles.contains(&user_role_id) {
            // The user already has the id, no need to assign it again.
            return Ok(user);
        } else {
            // Because the user does not have the role, it needs to be added to to its list
            let mut new_roles = user.roles.clone();
            new_roles.push(user_role_id);

            let target = users.filter(
                users::uuid.eq(user_uuid.0),
            );
            diesel::update(target)
                .set(roles.eq(new_roles))
                .get_result(conn)
                .map_err(User::handle_error)
        }
    }

    /// Gets a number of users at specified offsets.
    pub fn get_paginated(page_index: i32, page_size: i32, conn: &PgConnection) -> JoeResult<(Vec<User>, i64)> {
        use crate::schema::users;
        use crate::diesel_extensions::pagination::Paginate;

        users::table
            .order(users::user_name)
            .paginate(page_index.into())
            .per_page(page_size.into())
            .load_and_count_pages::<User>(conn)
            .map_err(User::handle_error)
    }

    /// Updates the user's display name.
    pub fn update_user_display_name(current_user_name: String, new_display_name: String, conn: &PgConnection) -> JoeResult<User> {
        use crate::schema::users::dsl::*;

        let target = users.filter(
            user_name.eq(current_user_name),
        );

        info!("Updating user display name");
        diesel::update(target)
            .set(display_name.eq(
                new_display_name,
            ))
            .get_result(conn)
            .map_err(User::handle_error)
    }

    // TODO deprecate the update user display name and switch to this impl, replacing the name.
    pub fn update_user_display_name_safe(user_uuid: UserUuid, new_display_name: String, conn: &PgConnection) -> JoeResult<User> {
        use crate::schema::users::dsl::*;

        let target = users.filter(
            uuid.eq(user_uuid.0),
        );

        info!("Updating user display name");
        diesel::update(target)
            .set(display_name.eq(
                new_display_name,
            ))
            .get_result(conn)
            .map_err(User::handle_error)
    }

    /// Deletes the user by their name.
    pub fn delete_user_by_name(name: String, conn: &PgConnection) -> JoeResult<User> {
        use crate::schema::users::dsl::*;

        let target = users.filter(user_name.eq(name));

        diesel::delete(target)
            .get_result(conn)
            .map_err(User::handle_error)
    }
}

