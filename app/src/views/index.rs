use macros::route_wrapper;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use crate::auth::{require_login, LoginGuard};
use crate::errors::AppError;
use crate::errors::Result;
use crate::state::AppState;

pub(super) fn route(state: AppState) -> BoxedFilter<(impl Reply,)> {
    let state = with_state!(state);

    let hello = warp::get().and(warp::path("hello")).and_then(hello);

    let hello_error = warp::get()
        .and(warp::path("hello_error"))
        .and_then(hello_error);

    let hello_require_login = warp::get()
        .and(warp::path("hello_require_login"))
        .and(require_login())
        .and_then(hello_require_login);

    let hello_with_state = warp::get()
        .and(warp::path("hello_with_state"))
        .and(state)
        .and_then(hello_with_state);

    let route = warp::any().and(
        hello
            .or(hello_error)
            .or(hello_require_login)
            .or(hello_with_state),
    );

    route.boxed()
}

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
