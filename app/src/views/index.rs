use warp::filters::BoxedFilter;
use warp::hyper::StatusCode;
use warp::Filter;
use warp::Reply;

use macros::route_wrapper;

use crate::auth::{require_login, LoginGuard};
use crate::errors::AppError;
use crate::errors::Result;
use crate::state::AppState;

macro_rules! declare_error {
    ($err:ty, $($arg:tt)*) => {
        pub async fn recover_error(
            rejection: warp::reject::Rejection,
        ) -> std::result::Result<warp::reply::Response, warp::reject::Rejection> {
            let mut resp = warp::reply::Response::default();

            if let Some(app_error) = rejection.find::<$err>() {
                // convert your error into http response
                *resp.body_mut() = warp::hyper::Body::from(app_error.to_string());
                *resp.status_mut() = match app_error {
                    $($arg)*
                };

                Ok(resp)
            } else {
                Err(rejection)
            }
        }
    };
}

declare_error!(IndexError,
    IndexError::A => {
        StatusCode::FORBIDDEN
    },
    IndexError::B => StatusCode::OK,
);

pub(super) fn route(state: AppState) -> BoxedFilter<(impl Reply,)> {
    let with_state = with_state!(state);

    let hello = warp::get().and(warp::path("hello")).and_then(hello);

    let hello_error = warp::get()
        .and(warp::path("hello_error"))
        .and_then(hello_error);

    let hello_require_login = warp::get()
        .and(warp::path("hello_require_login"))
        .and(require_login(state.clone()))
        .and_then(hello_require_login);

    let hello_with_state = warp::get()
        .and(warp::path("hello_with_state"))
        .and(with_state)
        .and_then(hello_with_state);

    let index_error = warp::get()
        .and(warp::path("index_error"))
        .and(require_login(state.clone()))
        .and_then(index_error);

    let route = warp::any()
        .and(
            hello
                .or(hello_error)
                .or(hello_require_login)
                .or(hello_with_state)
                .or(index_error),
        )
        .recover(recover_error);

    route.boxed()
}

#[derive(Debug, thiserror::Error)]
enum IndexError {
    #[error("A")]
    A,
    #[error("B")]
    B,
}

impl warp::reject::Reject for IndexError {}

#[route_wrapper]
async fn hello() -> Result<&'static str> {
    Ok("hello world")
}

#[route_wrapper]
async fn hello_error() -> Result<&'static str> {
    Err(AppError::MyError("my error".into()))
}

/// note: must use `_guard: LoginGuard` instead `_: LoginGuard`
#[route_wrapper]
async fn hello_require_login(_guard: LoginGuard) -> Result<&'static str> {
    Ok("you are logged in")
}

#[route_wrapper]
async fn hello_with_state(state: AppState) -> Result<&'static str> {
    let _ = state.db.query();

    Ok("hello with state")
}


#[route_wrapper]
async fn index_error(_g: LoginGuard) -> Result<&'static str, IndexError> {
    Err(IndexError::A)
}
