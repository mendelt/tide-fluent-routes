//! Tide Fluent Routes is a fluent api to define routes for the Tide HTTP framework.
//! At the moment it supports setting up paths, you can integrate middleware at any place in the
//! route-tree and you can integrate endpoints.
//! Some things that are possible with Tide-native routes are not (yet) possible;
//! - middleware does not work for now, there is support missing for this in Tide
//! - Tide prefix routes are not implemented
//! - you can not nest Tide servers
//! - serving directories is not possible but a version of this is planned
//!
//! To use this you can import Tide Fluent Routes with `use tide_fluent_routes::prelude::* it
//! introduces the `register` extension method on the `Tide::Server to register routes from a
//! RouteBuilder.
//! A RouteBuilder can be initialized using the `route()` method.
//! You can register simple endpoints like this;
//! ```rust
//! # use tide::{Request, Result};
//! # use tide_fluent_routes::prelude::*;
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
//! Fluent Routes follows conventions from Tide. All HTTP verbs are supported the same way. Paths
//! can be extended using `at` but this method takes a router closure that allows building the route
//! as a tree.
//! A complete route tree can be defined like this;
//! ```rust
//! # use tide::{Request, Result};
//! # use tide_fluent_routes::prelude::*;
//! # async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! # let mut server = tide::Server::new();
//!
//! server.register(
//!     root()
//!         .get(endpoint)
//!         .post(endpoint)
//!         .at("api/v1", |route| route
//!             .get(endpoint)
//!             .post(endpoint)
//!         )
//!         .at("api/v2", |route| route
//!             .get(endpoint)
//!             .post(endpoint)
//!         ),
//! );
//! ```
//! This eliminates the need to introduce variables for partial pieces of your route tree.
//!
//! Another problem with Tide routes is that middleware that is only active for certain routes can
//! be difficult to maintain. Adding middleware to a tree is easy, and its very clear where the
//! middleware is applied and where not; (this is still a prototype and middleware is not actually
//! added right now)
//! ```rust
//! # use std::{future::Future, pin::Pin};
//! # use tide::{Next, Request, Result};
//! # use tide_fluent_routes::prelude::*;
//! # async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! # fn dummy_middleware<'a>(
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
//!         .at("api/v1", |route| route
//!             .with(dummy_middleware, |route| route
//!                 .get(endpoint)
//!             )
//!             .post(endpoint)
//!         )
//!         .at("api/v2", |route| route
//!             .get(endpoint)
//!             .get(endpoint)
//!         ),
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

mod path;
pub mod routebuilder;
pub mod router;
mod util;

use crate::path::Path;
use crate::util::{ArcMiddleware, BoxedEndpoint};
use routebuilder::RouteBuilder;
use std::collections::HashMap;
use tide::http::Method;
use tide::{Endpoint, Middleware};

/// Start building a route. Returns a RouteBuilder for the root of your route
pub fn root<State>() -> RouteSegment<State> {
    RouteSegment {
        route: RouteSegmentKind::Root,
        name: None,
        branches: Vec::new(),
        endpoints: HashMap::new(),
    }
}

/// A Builder for Tide routes. RouteBuilders can be composed into a tree that represents the tree of
/// path segments, middleware and endpoints that defines the routes in a Tide application. This tree
/// can then be returned as a list of routes to each of the endpoints.
#[derive(Debug)]
pub struct RouteSegment<State> {
    route: RouteSegmentKind<State>,
    name: Option<String>,
    branches: Vec<RouteSegment<State>>,
    endpoints: HashMap<Option<Method>, BoxedEndpoint<State>>,
}

impl<State: Clone + Send + Sync + 'static> RouteSegment<State> {
    /// Add a branch, helper method for at and with methods
    fn add_branch<R: Fn(Self) -> Self>(mut self, spec: RouteSegmentKind<State>, routes: R) -> Self {
        self.branches.push(routes(RouteSegment {
            route: spec,
            name: None,
            branches: Vec::new(),
            endpoints: HashMap::new(),
        }));
        self
    }

    fn build(self) -> Vec<EndpointDescriptor<State>> {
        let local_endpoints: Vec<EndpointDescriptor<State>> = self
            .endpoints
            .into_iter()
            .map(|(method, endpoint)| EndpointDescriptor(Path::new(), method, Vec::new(), endpoint))
            .collect();

        let sub_endpoints: Vec<EndpointDescriptor<State>> = self
            .branches
            .into_iter()
            .flat_map(RouteSegment::build)
            .collect();

        let route = self.route;
        local_endpoints
            .into_iter()
            .chain(sub_endpoints.into_iter())
            .map(|descriptor| route.clone().apply_to(descriptor))
            .collect()
    }
}

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> for RouteSegment<State> {
    fn at<R: Fn(Self) -> Self>(self, path: &str, routes: R) -> Self {
        self.add_branch(RouteSegmentKind::Path(path.to_string()), routes)
    }

    fn with<M: Middleware<State>, R: Fn(Self) -> Self>(self, middleware: M, routes: R) -> Self {
        self.add_branch(
            RouteSegmentKind::Middleware(ArcMiddleware::new(middleware)),
            routes,
        )
    }

    fn method(mut self, method: Method, endpoint: impl Endpoint<State>) -> Self {
        self.endpoints
            .insert(Some(method), BoxedEndpoint::new(endpoint));
        self
    }

    fn all(mut self, endpoint: impl Endpoint<State>) -> Self {
        self.endpoints.insert(None, BoxedEndpoint::new(endpoint));
        self
    }

    fn name(mut self, name: &str) -> Self {
        if self.name.is_some() {
            panic!("route already has a name");
        }
        self.name = Some(name.to_string());
        self
    }
}

#[derive(Debug, Clone)]
enum RouteSegmentKind<State> {
    Root,
    Path(String),
    Middleware(ArcMiddleware<State>),
}

impl<State> RouteSegmentKind<State> {
    /// Apply the path or middleware in to the endpoint
    fn apply_to(self, endpoint: EndpointDescriptor<State>) -> EndpointDescriptor<State> {
        let EndpointDescriptor(path, method, mut middleware, endpoint) = endpoint;

        match self {
            RouteSegmentKind::Root => EndpointDescriptor(path, method, middleware, endpoint),
            RouteSegmentKind::Path(segment) => {
                EndpointDescriptor(path.prepend(&segment), method, middleware, endpoint)
            }
            RouteSegmentKind::Middleware(ware) => {
                middleware.push(ware);
                EndpointDescriptor(path, method, middleware, endpoint)
            }
        }
    }
}

/// Describes all information for registering an endpoint, the path to it, its middleware
/// and its HttpMethod
#[derive(Debug)]
pub(crate) struct EndpointDescriptor<State>(
    Path,
    Option<Method>,
    Vec<ArcMiddleware<State>>,
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
    use super::ArcMiddleware;
    use std::future::Future;
    use std::pin::Pin;
    use tide::{Next, Request, Result};

    #[test]
    fn should_build_single_endpoint() {
        let routes: Vec<_> = root::<()>().get(|_| async { Ok("") }).build();

        assert_eq!(routes.len(), 1);
    }

    #[test]
    fn should_build_multiple_endpoints() {
        let routes: Vec<_> = root::<()>()
            .get(|_| async { Ok("") })
            .post(|_| async { Ok("") })
            .build();

        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn should_build_sub_endpoints() {
        let routes: Vec<_> = root::<()>()
            .at("sub_path", |r| {
                r.get(|_| async { Ok("") }).post(|_| async { Ok("") })
            })
            .build();

        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn should_build_endpoint_path() {
        let routes: Vec<_> = root::<()>()
            .at("path", |r| r.at("subpath", |r| r.get(|_| async { Ok("") })))
            .build();

        assert_eq!(routes.len(), 1);
        assert_eq!(routes.get(0).unwrap().1, Some(Method::Get));
        assert_eq!(
            routes.get(0).unwrap().0.to_string(),
            "path/subpath".to_string()
        );
    }

    fn middleware<'a>(
        request: Request<()>,
        next: Next<'a, ()>,
    ) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
        Box::pin(async { Ok(next.run(request).await) })
    }

    #[test]
    fn should_collect_middleware() {
        let middleware1 = ArcMiddleware::new(middleware);
        let middleware2 = ArcMiddleware::new(middleware);

        let routes: Vec<_> = root::<()>()
            .at("path", |r| {
                r.with(middleware1.clone(), |r| {
                    r.at("subpath", |r| {
                        r.with(middleware2.clone(), |r| r.get(|_| async { Ok("") }))
                    })
                    .get(|_| async { Ok("") })
                })
            })
            .build();

        assert_eq!(routes.get(0).unwrap().2.len(), 1);
        assert_eq!(routes.get(1).unwrap().2.len(), 2);
    }
}
