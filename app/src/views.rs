use warp::hyper::{http::Request, service::Service, Body};
use warp::{serve, service};
use warp::{Filter, Reply, Server};

use crate::errors::recover_error;
use crate::state::AppState;

mod index;

/// create main filter
fn build_filter(state: AppState) -> impl Filter<Extract = impl Reply> + Clone {
    warp::any()
        .and(
            index::route(state.clone()),
            // .or(foo::route())
            // .or(bar::route())
        )
        .recover(recover_error)
}

#[allow(dead_code)]
pub fn build_service(state: AppState) -> impl Service<Request<Body>> {
    let route = build_filter(state);

    service(route)
}

#[allow(dead_code)]
pub fn build_server(state: AppState) -> Server<impl Filter<Extract = impl Reply> + Clone> {
    let route = build_filter(state);

    serve(route)
}
