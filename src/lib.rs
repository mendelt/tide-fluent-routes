use std::collections::HashMap;
use tide::{http::Method, Endpoint, Middleware};

pub trait Router<State: Clone + Send + Sync + 'static> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>);

    fn register(&mut self, _routes: RouteBuilder<State>) {
        todo!()
    }
}

impl<State: Clone + Send + Sync + 'static> Router<State> for tide::Server<State> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>) {
        self.at(path).method(method, endpoint);
    }
}

pub fn root<State>() -> RouteBuilder<State> {
    RouteBuilder {
        route: RouteSpecifier::Root,
        branches: Vec::new(),
        endpoints: HashMap::new(),
    }
}

pub struct RouteBuilder<State> {
    route: RouteSpecifier<State>,

    branches: Vec<RouteBuilder<State>>,
    endpoints: HashMap<Method, Box<dyn Endpoint<State>>>,
}

pub enum RouteSpecifier<State> {
    Root,
    Path(String),
    Middleware(Box<dyn Middleware<State>>),
}

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> {
    pub fn at<R: Fn(Self) -> Self>(self, path: &str, routes: R) -> Self {
        self.add_branch(RouteSpecifier::Path(path.to_string()), routes)
    }

    pub fn with<M: Middleware<State>, R: Fn(Self) -> Self>(self, middleware: M, routes: R) -> Self {
        self.add_branch(RouteSpecifier::Middleware(Box::new(middleware)), routes)
    }

    fn add_branch<R: Fn(Self) -> Self>(mut self, spec: RouteSpecifier<State>, routes: R) -> Self {
        self.branches.push(routes(RouteBuilder {
            route: spec,
            branches: Vec::new(),
            endpoints: HashMap::new(),
        }));
        self
    }

    pub fn method(mut self, method: Method, endpoint: impl Endpoint<State>) -> Self {
        self.endpoints.insert(method, Box::new(endpoint));
        self
    }
}

#[cfg(test)]
mod test {
    use std::{future::Future, pin::Pin};

    use super::*;
    use tide::{Next, Request, Result};

    struct StubRouter {}
    impl Router<()> for StubRouter {
        fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<()>) {
            todo!()
        }
    }

    async fn endpoint(_: Request<()>) -> Result {
        todo!()
    }

    fn dummy_middleware<'a>(
        request: Request<()>,
        next: Next<'a, ()>,
    ) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
        Box::pin(async { Ok(next.run(request).await) })
    }

    #[test]
    fn example_build_basic_route() {
        let mut router = StubRouter {};

        router.register(
            root()
                .method(Method::Get, endpoint)
                .method(Method::Post, endpoint),
        );
    }

    #[test]
    fn example_build_nested_route() {
        let mut router = StubRouter {};

        router.register(
            root()
                .method(Method::Get, endpoint)
                .method(Method::Post, endpoint)
                .at("api/v1", |route| {
                    route
                        .method(Method::Get, endpoint)
                        .method(Method::Post, endpoint)
                })
                .at("api/v2", |route| {
                    route
                        .method(Method::Get, endpoint)
                        .method(Method::Post, endpoint)
                }),
        );
    }

    #[test]
    fn example_build_middleware_route() {
        let mut router = StubRouter {};

        router.register(
            root()
                .method(Method::Get, endpoint)
                .method(Method::Post, endpoint)
                .at("api/v1", |route| {
                    route
                        .with(dummy_middleware, |route| {
                            route.method(Method::Get, endpoint)
                        })
                        .method(Method::Post, endpoint)
                })
                .at("api/v2", |route| {
                    route
                        .method(Method::Get, endpoint)
                        .method(Method::Post, endpoint)
                }),
        );
    }
}
