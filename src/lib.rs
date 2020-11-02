//! Tide Fluent Routes is a fluent api to define routes for the Tide HTTP framework.
//! At the moment it supports setting up paths, you can integrate middleware at any place in the
//! route-tree and you can integrate endpoints.
//! Some things that are possible with Tide-native routes are not (yet) possible;
//! - Tide prefix routes are not implemented
//! - you can not nest Tide servers
//!
//! To use this you can import Tide Fluent Routes with `use tide_fluent_routes::prelude::* it
//! introduces the `register` extension method on the `Tide::Server to register routes from a
//! RouteBuilder.
//! A RouteBuilder can be initialized using the `route()` method.
//! You can register simple endpoints like this;
//! ```rust
//! # use tide::{Request, Result};
//! # pub async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! use tide_fluent_routes::prelude::*;
//!
//! let mut server = tide::Server::new();
//!
//! server.register(
//!    root()
//!        .get(endpoint)
//!        .post(endpoint));
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
//!         )
//! );
//! ```
//! This eliminates the need to introduce variables for partial pieces of your route tree.
//!
//! Including routes defined in other functions also looks very natural, this makes it easy
//! to compose large route trees from smaller trees defined elsewhere;
//! ```rust
//! # use tide::{Request, Result};
//! # use tide_fluent_routes::prelude::*;
//! # async fn endpoint(_: Request<()>) -> Result {
//! #     todo!()
//! # }
//! # let mut server = tide::Server::new();
//!
//! fn v1_routes(routes: RouteSegment<()>) -> RouteSegment<()> {
//!     routes
//!         .at("articles", |route| route
//!             .get(endpoint)
//!             .post(endpoint)
//!             .at(":id", |route| route
//!                 .get(endpoint)
//!                 .put(endpoint)
//!                 .delete(endpoint)
//!             )
//!         )
//! }
//!
//! fn v2_routes(routes: RouteSegment<()>) -> RouteSegment<()> {
//!     routes
//!         .at("articles", |route| route
//!             .get(endpoint))
//! }
//!
//! server.register(
//!     root()
//!         .get(endpoint)
//!         .post(endpoint)
//!         .at("api/v1", v1_routes)
//!         .at("api/v2", v2_routes));
//! ```
//!
//! With vanilla Tide routes it can be hard to see what middleware is active for what
//! endpoints.
//! Adding middleware to a tree is easy, and its very clear where the middleware is applied;
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
//!
//! Serving directories is possible using `serve_dir`, this works the same as with normal Tide routes,
//! fluent routes adds the `serve_file` convenience method for serving single files.
//! ```rust,no_run
//! # use tide::{Request, Result};
//! use tide_fluent_routes::prelude::*;
//! use tide_fluent_routes::fs::ServeFs;
//!
//! let mut server = tide::Server::new();
//!
//! server.register(
//!     root()
//!         .serve_file("files/index.html").unwrap()
//!         .at("img", |r| r
//!             .serve_dir("files/images").unwrap()
//!         )
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

pub mod fs;
mod path;
pub mod reverse_router;
pub mod routebuilder;
pub mod router;
mod util;

use crate::path::Path;
use crate::util::{ArcMiddleware, BoxedEndpoint};
use reverse_router::ReverseRouter;
use routebuilder::RouteBuilder;
use std::collections::HashMap;
use tide::http::Method;
use tide::{Endpoint, Middleware};

/// Start building a route. Returns a RouteBuilder for the root of your route
pub fn root<State>() -> RouteSegment<State> {
    RouteSegment {
        path: Path::prefix("/"),
        middleware: Vec::new(),
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
    path: Path,
    middleware: Vec<ArcMiddleware<State>>,

    name: Option<String>,
    branches: Vec<RouteSegment<State>>,
    endpoints: HashMap<Option<Method>, BoxedEndpoint<State>>,
}

impl<State: Clone + Send + Sync + 'static> RouteSegment<State> {
    fn names(&self) -> Vec<RouteDescriptor<State>> {
        let path = self.path.clone();

        let local_name = self
            .name
            .clone()
            .map(|name| RouteDescriptor {
                path: path.clone(),
                middleware: Vec::new(), // We don't care about middleware for route names
                route: Route::Name(name),
            })
            .into_iter();

        let sub_routes = self.branches.iter().flat_map(RouteSegment::names);

        local_name.chain(sub_routes).collect()
    }

    /// Construct a reverse router for the paths in the route builder
    pub fn reverse_router(&self) -> ReverseRouter {
        let mut routes = ReverseRouter::new();

        for RouteDescriptor {
            path,
            middleware: _,
            route,
        } in self.names()
        {
            if let Route::Name(name) = route {
                routes.insert(&name, &path.to_string());
            }
        }

        routes
    }

    fn build(self) -> Vec<RouteDescriptor<State>> {
        let path = self.path;
        let middleware = self.middleware;

        let local_endpoints =
            self.endpoints
                .into_iter()
                .map(|(method, endpoint)| RouteDescriptor {
                    path: path.clone(),
                    middleware: middleware.clone(),
                    route: Route::Handler(method, endpoint),
                });

        let sub_endpoints = self.branches.into_iter().flat_map(RouteSegment::build);

        local_endpoints.chain(sub_endpoints).collect()
    }
}

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> for RouteSegment<State> {
    fn at<R: FnOnce(Self) -> Self>(mut self, path: &str, routes: R) -> Self {
        self.branches.push(routes(RouteSegment {
            path: self.path.clone().append(path),
            middleware: self.middleware.clone(),
            name: None,
            branches: Vec::new(),
            endpoints: HashMap::new(),
        }));
        self
    }

    fn with<M: Middleware<State>, R: FnOnce(Self) -> Self>(
        mut self,
        middleware: M,
        routes: R,
    ) -> Self {
        let mut ware = self.middleware.clone();
        ware.push(ArcMiddleware::new(middleware));

        self.branches.push(routes(RouteSegment {
            path: self.path.clone(),
            middleware: ware,
            name: None,
            branches: Vec::new(),
            endpoints: HashMap::new(),
        }));
        self
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
        if let Some(name) = self.name {
            panic!("route already has name: {}", name);
        }
        self.name = Some(name.to_string());
        self
    }
}

/// Describes a branch in the route tree, the path and middleware collected and the route as the leaf
#[derive(Debug)]
pub(crate) struct RouteDescriptor<State> {
    path: Path,
    middleware: Vec<ArcMiddleware<State>>,
    route: Route<State>,
}

/// Descibes a leaf in the route tree, either a name or a handler
#[derive(Debug)]
pub(crate) enum Route<State> {
    Name(String),
    Handler(Option<Method>, BoxedEndpoint<State>),
}

/// Import types to use tide_fluent_routes
pub mod prelude {
    pub use super::reverse_router::ReverseRouter;
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
        // TODO: Fix this, possibly with a named endpoint
        // assert_eq!(routes.get(0).unwrap().route, Some(Method::Get));
        assert_eq!(
            routes.get(0).unwrap().path.to_string(),
            "/path/subpath".to_string()
        );
    }

    #[test]
    fn should_start_path_with_slash() {
        let routes: Vec<_> = root::<()>().get(|_| async { Ok("") }).build();
        assert_eq!(routes.get(0).unwrap().path.to_string(), "/".to_string());
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

        assert_eq!(routes.get(0).unwrap().middleware.len(), 1);
        assert_eq!(routes.get(1).unwrap().middleware.len(), 2);
    }
}
