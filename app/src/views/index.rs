use macros::route_wrapper;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

use crate::errors::AppError;
use crate::errors::Result;

pub(super) fn route() -> BoxedFilter<(impl Reply, )> {
    let hello = warp::get().and(warp::path("hello")).and_then(hello);

    let hello_error = warp::get()
        .and(warp::path("hello_error"))
        .and_then(hello_error);

    let route = warp::any().and(hello.or(hello_error));

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
