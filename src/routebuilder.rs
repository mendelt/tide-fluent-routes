//! The RouteBuilder trait defines the internal dsl to build route trees as implemented by all
//! RouteSegments

use tide::{Endpoint, Middleware};
use tide::http::Method;

/// A routebuilder can be used to define routes by adding path segments, middelwares and endpoints
/// to a route tree
pub trait RouteBuilder<State: Clone + Send + Sync + 'static>: Sized {
    /// Add a path segment with a set of sub-routes
    fn at<R: Fn(Self) -> Self>(self, path: &str, routes: R) -> Self;

    /// Add middleware with a set of sub-routes
    fn with<M: Middleware<State>, R: Fn(Self) -> Self>(self, middleware: M, routes: R) -> Self;

    /// Add an endpoint for an http method
    fn method(self, method: Method, endpoint: impl Endpoint<State>) -> Self;

    /// Add a catchall endpoint
    fn all(self, endpoint: impl Endpoint<State>) -> Self;
}

/// Some extension methods for the routebuilder to make the routing dsl a bit nicer
pub trait RouteBuilderExt<State: Clone + Send + Sync + 'static> : RouteBuilder<State> {
    /// Add an HTTP GET endpoint
    fn get(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Get, endpoint)
    }

    /// Add an HTTP POST endpoint
    fn post(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Post, endpoint)
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> RouteBuilderExt<State> for R {}
