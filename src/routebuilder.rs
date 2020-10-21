//! The RouteBuilder trait defines the internal dsl to build route trees as implemented by all
//! RouteSegments

use tide::http::Method;
use tide::{Endpoint, Middleware};

/// A routebuilder can be used to define routes by adding path segments, middelwares and endpoints
/// to a route tree
pub trait RouteBuilder<State: Clone + Send + Sync + 'static>: Sized {
    /// Add a path segment with a set of sub-routes
    fn at<R: FnOnce(Self) -> Self>(self, path: &str, routes: R) -> Self;

    /// Add middleware with a set of sub-routes
    fn with<M: Middleware<State>, R: FnOnce(Self) -> Self>(self, middleware: M, routes: R) -> Self;

    /// Add an endpoint for an http method
    fn method(self, method: Method, endpoint: impl Endpoint<State>) -> Self;

    /// Add a catchall endpoint
    fn all(self, endpoint: impl Endpoint<State>) -> Self;

    /// Make this a named route
    fn name(self, name: &str) -> Self;
}

/// Some extension methods for the routebuilder to make the routing dsl a bit nicer
pub trait RouteBuilderExt<State: Clone + Send + Sync + 'static>: RouteBuilder<State> {
    /// Add an HTTP GET endpoint
    fn get(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Get, endpoint)
    }

    /// Add an HTTP HEAD endpoint
    fn head(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Head, endpoint)
    }

    /// Add an HTTP PUT endpoint
    fn put(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Put, endpoint)
    }

    /// Add an HTTP POST endpoint
    fn post(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Post, endpoint)
    }

    /// Add an HTTP DELETE endpoint
    fn delete(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Delete, endpoint)
    }

    /// Add an HTTP OPTIONS endpoint
    fn options(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Options, endpoint)
    }

    /// Add an HTTP CONNECT endpoint
    fn connect(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Connect, endpoint)
    }

    /// Add an HTTP PATCH endpoint
    fn patch(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Patch, endpoint)
    }

    /// Add an HTTP TRACE endpoint
    fn trace(self, endpoint: impl Endpoint<State>) -> Self {
        self.method(Method::Trace, endpoint)
    }
}

impl<State: Clone + Send + Sync + 'static, R: RouteBuilder<State>> RouteBuilderExt<State> for R {}
