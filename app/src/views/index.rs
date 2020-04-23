use macros::route_wrapper;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use crate::errors::AppError;
use crate::errors::Result;
use crate::auth::{LoginGuard, require_login};

pub(super) fn route() -> BoxedFilter<(impl Reply, )> {
    let hello = warp::get().and(warp::path("hello")).and_then(hello);

    let hello_error = warp::get()
        .and(warp::path("hello_error"))
        .and_then(hello_error);

    let hello_require_login = warp::get()
        .and(warp::path("hello_require_login"))
        .and(require_login())
        .and_then(hello_require_login);

    let route = warp::any().and(hello.or(hello_error).or(hello_require_login));

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