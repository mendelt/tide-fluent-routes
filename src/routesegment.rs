use crate::Result;
use crate::path::Path;
use crate::util::{ArcMiddleware, BoxedEndpoint};
use crate::reverse_router::ReverseRouter;
use crate::routebuilder::RouteBuilder;
use std::collections::HashMap;
use tide::http::Method;
use tide::{Endpoint, Middleware};

/// Start building a route. Returns a RouteSegment for the root of your route
pub fn root<State>() -> SubRoute<State> {
    Ok(RouteSegment {
        path: Path::prefix("/"),
        middleware: Vec::new(),
        name: None,
        branches: Vec::new(),
        endpoints: HashMap::new(),
    })
}

/// A segment of a tide route tree. RouteSegments can be composed into trees that represents a tree of
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

    pub(crate) fn build(self) -> Vec<RouteDescriptor<State>> {
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

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> for Result<RouteSegment<State>> {
    fn at<R: FnOnce(Self) -> Self>(self, path: &str, routes: R) -> Self {
        match self {
            Ok(mut segment) => {
                segment.branches.push(routes(Ok(RouteSegment {
                    path: segment.path.clone().append(path),
                    middleware: segment.middleware.clone(),
                    name: None,
                    branches: Vec::new(),
                    endpoints: HashMap::new(),
                }))?);
                Ok(segment)
            }
            Err(e) => Err(e),
        }
    }

    fn with<M: Middleware<State>, R: FnOnce(Self) -> Self>(self, middleware: M, routes: R) -> Self {
        match self {
            Ok(mut segment) => {
                let mut ware = segment.middleware.clone();
                ware.push(ArcMiddleware::new(middleware));

                segment.branches.push(routes(Ok(RouteSegment {
                    path: segment.path.clone(),
                    middleware: ware,
                    name: None,
                    branches: Vec::new(),
                    endpoints: HashMap::new(),
                }))?);
                Ok(segment)
            }
            Err(e) => Err(e),
        }
    }

    fn method(self, method: Method, endpoint: impl Endpoint<State>) -> Self {
        match self {
            Ok(mut segment) => {
                segment
                    .endpoints
                    .insert(Some(method), BoxedEndpoint::new(endpoint));
                Ok(segment)
            }
            Err(e) => Err(e),
        }
    }

    fn all(self, endpoint: impl Endpoint<State>) -> Self {
        match self {
            Ok(mut segment) => {
                segment.endpoints.insert(None, BoxedEndpoint::new(endpoint));
                Ok(segment)
            }
            Err(e) => Err(e),
        }
    }

    fn name(self, name: &str) -> Self {
        match self {
            Ok(mut segment) => {
                if let Some(name) = segment.name {
                    panic!("route already has name: {}", name);
                }
                segment.name = Some(name.to_string());
                Ok(segment)
            }
            Err(e) => Err(e),
        }
    }
}

/// Partial routing results for passing around in routing closures
pub type SubRoute<T> = Result<RouteSegment<T>>;


/// Describes a branch in the route tree, the path and middleware collected and the route as the leaf
#[derive(Debug)]
pub(crate) struct RouteDescriptor<State> {
    pub(crate) path: Path,
    pub(crate) middleware: Vec<ArcMiddleware<State>>,
    pub(crate) route: Route<State>,
}

/// Descibes a leaf in the route tree, either a name or a handler
#[derive(Debug)]
pub(crate) enum Route<State> {
    Name(String),
    Handler(Option<Method>, BoxedEndpoint<State>),
}