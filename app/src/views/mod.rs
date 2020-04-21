use warp::filters::BoxedFilter;
use warp::hyper::{http::Request, service::Service, Body};
use warp::{serve, service};
use warp::{Filter, Reply, Server};

use crate::errors::recover_error;

mod index;

/// create main filter
fn build_filter() -> BoxedFilter<(impl Reply,)> {
    warp::any()
        .and(
            index::route(), // .or(foo::route())
                            // .or(bar::route())
        )
        .recover(recover_error)
        .boxed()
}

#[allow(dead_code)]
pub fn build_service() -> impl Service<Request<Body>> {
    let route = build_filter();

    service(route)
}

#[allow(dead_code)]
pub fn build_server() -> Server<BoxedFilter<(impl Reply,)>> {
    let route = build_filter();

    serve(route)
}
