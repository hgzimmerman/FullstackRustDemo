use chrono::{NaiveDateTime, Utc};
use chrono::Duration;
use crate::auth_lib::ServerJwt;
use crate::auth_lib::Secret;
use identifiers::user::UserUuid;
use wire::user::Jwt;
use crate::auth_lib::verify_hash;
use crate::user::User;
use wire::login::LoginRequest;
use error::LoginError;
use diesel::PgConnection;

use log::info;

use error::LoginResult;


pub fn login(login_request: LoginRequest, secret: &Secret, conn: &PgConnection) -> LoginResult {
    info!("Logging in for user: {}", &login_request.user_name);

    let user: User = User::get_user_by_user_name(&login_request.user_name, &conn)
        .map_err(|_| LoginError::UsernameDoesNotExist)?;

    // Check if the user is locked.
    // This will clean up any locked status if the lock has already expired.
    if user.check_if_locked(conn).map_err(
        |_| {
            info!("Db error while checking for locks.");
            LoginError::OtherError("DB error")
        },
    )?
    {
        info!("Account locked.");
        return Err(LoginError::AccountLocked);
    }

    let user_uuid = UserUuid(user.uuid);
    info!("Verifying password against hash");
    match verify_hash(&login_request.password, &user.password_hash) {
        Ok(b) => {
            if !b {
                info!("Wrong password entered for user: {}", &login_request.user_name);
                User::record_failed_login(user_uuid, user.failed_login_count, &conn)
                    .map_err(|_| LoginError::OtherError("Login failed, but could not set the login delay"))?;
                return Err(LoginError::IncorrectPassword);
            } else {
                info!("Password match verified");
                if user.failed_login_count > 0 {
                    info!("Resetting login count");
                    User::reset_login_failure_count(user_uuid, &conn)
                        .map_err(|_| LoginError::OtherError("DB error"))?;
                }
            }
        }
        Err(e) => return Err(LoginError::PasswordHashingError(e)),
    }


    // generate token
    info!("Generating JWT Expiry Date");
    let duration: Duration = Duration::days(7); // Expire after a week
    let new_expire_date: NaiveDateTime = match Utc::now().checked_add_signed(duration) {
        Some(ndt) => ndt.naive_utc(),
        None => return Err(LoginError::OtherError("Could not calculate offset for token expiry")),
    };


    info!("Creating JWT");
    let jwt = Jwt {
        //        user_name: user.user_name.clone(),
        sub: UserUuid(user.uuid),
        user_roles: user.roles
            .iter()
            .map(|role_id| (*role_id).into())
            .collect(),
        exp: new_expire_date,
        iat: Utc::now().naive_utc(),
    };
    let jwt = ServerJwt(jwt);
    let jwt_string: String = match jwt.encode_jwt_string(&secret) {
        Ok(s) => s,
        Err(e) => return Err(LoginError::JwtError(e)),
    };

    Ok(jwt_string)
}


pub fn reauth(jwt: ServerJwt, secret: &Secret) -> LoginResult {
    let mut jwt = jwt.0;
    info!("Generating JWT Expiry Date");
    let duration: Duration = Duration::days(7); // Expire after a week
    let new_expire_date: NaiveDateTime = match Utc::now().checked_add_signed(duration) {
        Some(ndt) => ndt.naive_utc(),
        None => return Err(LoginError::OtherError("Could not calculate offset for token expiry")),
    };
    jwt.exp = new_expire_date;
    jwt.iat = Utc::now().naive_utc();

    let jwt = ServerJwt(jwt);
    let jwt_string: String = match jwt.encode_jwt_string(&secret) {
        Ok(s) => s,
        Err(e) => return Err(LoginError::JwtError(e)),
    };

    Ok(jwt_string)
}