//! The router trait and its implementation on tide::Server connect the RouteBuilder to tide and
//! allows you to call register on a tide::Server with a fluent route tree

use crate::{util::ArcMiddleware, Route, RouteDescriptor, RouteSegment};
use tide::{http::Method, Endpoint};

/// A router is any component where routes can be registered on like a tide::Server
pub trait Router<State: Clone + Send + Sync + 'static> {
    /// Register a single endpoint on the `Router`
    fn register_endpoint(
        &mut self,
        path: &str,
        method: Option<Method>,
        middleware: &[ArcMiddleware<State>],
        endpoint: impl Endpoint<State>,
    );

    /// Register all routes from a RouteBuilder on the `Router`
    fn register(&mut self, builder: RouteSegment<State>) -> &mut Self {
        for RouteDescriptor {
            path,
            middleware,
            route,
        } in builder.build()
        {
            if let Route::Handler(method, endpoint) = route {
                self.register_endpoint(&path.to_string(), method, &middleware, endpoint)
            }
        }

        self
    }
}

impl<State: Clone + Send + Sync + 'static> Router<State> for tide::Server<State> {
    fn register_endpoint(
        &mut self,
        path: &str,
        method: Option<Method>,
        middleware: &[ArcMiddleware<State>],
        endpoint: impl Endpoint<State>,
    ) {
        let mut route = self.at(path);
        for ware in middleware {
            route.with(ware.clone());
        }

        // if method is specified then register this method, otherwise register endpoint as a catch_all
        match method {
            Some(method) => self.at(path).method(method, endpoint),
            None => self.at(path).all(endpoint),
        };
    }
}
