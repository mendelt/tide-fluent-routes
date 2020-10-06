//! Tide Fluent Routes implements a fluent api to define your tide routes.
//!
//! You can register simple endpoints like this;
//! ```rust
//! # use tide::{Request, Result};
//! # use tide_fluent_routes::prelude::*;
//! #
//! # pub async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! use tide_fluent_routes::*;
//!
//! let mut server = tide::Server::new();
//!
//! server.register(
//!    root()
//!        .get(endpoint)
//!        .post(endpoint),
//! );
//! ```
//!
//! Or a more complete tree of urls and endpoints like this;
//! ```rust
//! # use tide::{Request, Result};
//! # use tide_fluent_routes::prelude::*;
//! #
//! # pub async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! #
//! # let mut server = tide::Server::new();
//!
//! server.register(
//!     root()
//!         .get(endpoint)
//!         .post(endpoint)
//!         .at("api/v1", |route| {
//!             route
//!                 .get(endpoint)
//!                 .post(endpoint)
//!         })
//!         .at("api/v2", |route| {
//!             route
//!                 .get(endpoint)
//!                 .post(endpoint)
//!         }),
//! );
//! ```
//!
//! Adding middleware is easy, and its very clear where the middleware is applied and where not;
//! ```rust
//! # use std::{future::Future, pin::Pin};
//! # use tide::{Next, Request, Result};
//! # use tide_fluent_routes::prelude::*;
//! #
//! # pub async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! #
//! # pub fn dummy_middleware<'a>(
//! #     request: Request<()>,
//! #     next: Next<'a, ()>,
//! # ) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
//! #     Box::pin(async { Ok(next.run(request).await) })
//! # }
//! # let mut server = tide::Server::new();
//! server.register(
//!     root()
//!         .get(endpoint)
//!         .post(endpoint)
//!         .at("api/v1", |route| {
//!             route
//!                 .with(dummy_middleware, |route| {
//!                     route.get(endpoint)
//!                 })
//!                .post(endpoint)
//!         })
//!         .at("api/v2", |route| {
//!             route
//!                 .get(endpoint)
//!                 .get(endpoint)
//!         }),
//! );
//! ```

// Turn on warnings for some lints
#![warn(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_qualifications
)]

pub mod routebuilder;
pub mod router;

use routebuilder::RouteBuilder;
use std::collections::HashMap;
use tide::http::Method;
use tide::utils::async_trait;
use tide::{Endpoint, Middleware};

struct BoxedEndpoint<State>(Box<dyn Endpoint<State>>);

impl<State: Clone + Send + Sync + 'static> BoxedEndpoint<State> {
    /// Wrap an endpoint in a BoxedEndpoint
    fn new(endpoint: impl Endpoint<State>) -> Self {
        Self(Box::new(endpoint))
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint<State> for BoxedEndpoint<State> {
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        self.0.call(req).await
    }
}

/// Start building a route. Returns a RouteBuilder for the root of your route
pub fn root<State>() -> RouteSegment<State> {
    RouteSegment {
        route: RouteSegmentKind::Root,
        branches: Vec::new(),
        endpoints: HashMap::new(),
    }
}

/// A Builder for Tide routes. RouteBuilders can be composed into a tree that represents the tree of
/// path segments, middleware and endpoints that defines the routes in a Tide application. This tree
/// can then be returned as a list of routes to each of the endpoints.
pub struct RouteSegment<State> {
    route: RouteSegmentKind<State>,
    branches: Vec<RouteSegment<State>>,
    endpoints: HashMap<Option<Method>, BoxedEndpoint<State>>,
}

enum RouteSegmentKind<State> {
    Root,
    Path(String),
    Middleware(Box<dyn Middleware<State>>),
}

impl<State: Clone + Send + Sync + 'static> RouteSegment<State> {
    /// Add a branch, helper method for at and with methods
    fn add_branch<R: Fn(Self) -> Self>(mut self, spec: RouteSegmentKind<State>, routes: R) -> Self {
        self.branches.push(routes(RouteSegment {
            route: spec,
            branches: Vec::new(),
            endpoints: HashMap::new(),
        }));
        self
    }

    fn build(self) -> impl Iterator<Item = EndpointDescriptor<State>> {
        let local_endpoints: Vec<EndpointDescriptor<State>> = self
            .endpoints
            .into_iter()
            .map(|(method, endpoint)| {
                EndpointDescriptor(String::new(), method, Vec::new(), endpoint)
            })
            .collect();

        let sub_endpoints: Vec<EndpointDescriptor<State>> = self
            .branches
            .into_iter()
            .flat_map(RouteSegment::build)
            .collect();

        local_endpoints.into_iter().chain(sub_endpoints.into_iter())
    }
}

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> for RouteSegment<State> {
    /// Add an endpoint
    fn method(mut self, method: Method, endpoint: impl Endpoint<State>) -> Self {
        self.endpoints
            .insert(Some(method), BoxedEndpoint::new(endpoint));
        self
    }

    fn all(mut self, endpoint: impl Endpoint<State>) -> Self {
        self.endpoints.insert(None, BoxedEndpoint::new(endpoint));
        self
    }

    /// Add sub-routes for a path segment
    fn at<R: Fn(Self) -> Self>(self, path: &str, routes: R) -> Self {
        self.add_branch(RouteSegmentKind::Path(path.to_string()), routes)
    }

    /// Add sub-routes for a middleware
    fn with<M: Middleware<State>, R: Fn(Self) -> Self>(self, middleware: M, routes: R) -> Self {
        self.add_branch(RouteSegmentKind::Middleware(Box::new(middleware)), routes)
    }
}

/// Describes all information for registering an endpoint, the path to it, its middleware
/// and its HttpMethod
struct EndpointDescriptor<State>(
    String,
    Option<Method>,
    Vec<Box<dyn Middleware<State>>>,
    BoxedEndpoint<State>,
);

/// Import types to use tide_fluent_routes
pub mod prelude {
    pub use super::routebuilder::{RouteBuilder, RouteBuilderExt};
    pub use super::router::Router;
    pub use super::{root, RouteSegment};
    pub use tide::http::Method;
}

#[cfg(test)]
mod test {
    use super::prelude::*;

    #[test]
    fn should_build_single_endpoint() {
        let routes: Vec<_> = root::<()>().get(|_| async { Ok("") }).build().collect();

        assert_eq!(routes.len(), 1);
    }

    #[test]
    fn should_build_multiple_endpoints() {
        let routes: Vec<_> = root::<()>()
            .get(|_| async { Ok("") })
            .post(|_| async { Ok("") })
            .build()
            .collect();

        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn should_build_sub_endpoints() {
        let routes: Vec<_> = root::<()>()
            .at("sub_path", |r| {
                r.get(|_| async { Ok("") }).post(|_| async { Ok("") })
            })
            .build()
            .collect();

        assert_eq!(routes.len(), 2);
    }
}
