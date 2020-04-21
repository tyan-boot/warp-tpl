use thiserror::Error;
use warp::hyper::http::StatusCode;
use warp::hyper::Body;
use warp::reject::{Reject, Rejection};
use warp::reply::Response;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("my error: {}", _0)]
    MyError(String),
}

impl Reject for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;

pub async fn recover_error(rejection: Rejection) -> std::result::Result<Response, Rejection> {
    let mut resp = Response::default();

    if let Some(app_error) = rejection.find::<AppError>() {
        // convert your error into http response
        *resp.status_mut() = StatusCode::OK;
        *resp.body_mut() = Body::from(app_error.to_string());

        Ok(resp)
    } else {
        Err(rejection)
    }
}
