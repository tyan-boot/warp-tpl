use warp::Filter;

use crate::errors::AppError;


/// put some `User` instead in LoginGuard;
pub struct LoginGuard(i64);

pub fn require_login() -> impl Filter<Extract=(LoginGuard, ), Error=warp::Rejection> + Clone {
    warp::header::optional("authorization")
        .and_then(|token: Option<String>| async move {
            let token = token.ok_or(AppError::Unauthorized).map_err(warp::reject::custom)?;

            // verify token...
            if token.as_str() == "remnant" {
                Ok(LoginGuard(1))
            } else {
                Err(warp::reject::custom(AppError::Unauthorized))
            }
        })
}