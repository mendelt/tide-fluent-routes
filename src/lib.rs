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
    pub fn at<R: Fn(RouteBuilder<State>) -> RouteBuilder<State>>(
        self,
        _path: &str,
        routes: R,
    ) -> RouteBuilder<State> {
        todo!()
    }

    pub fn with<M: Middleware<State>, R: Fn(RouteBuilder<State>) -> RouteBuilder<State>>(
        mut self,
        middleware: M,
        routes: R,
    ) -> RouteBuilder<State> {
        todo!()
    }

    pub fn method(mut self, method: Method, endpoint: impl Endpoint<State>) -> RouteBuilder<State> {
        self.endpoints.insert(method, Box::new(endpoint));
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tide::Request;

    struct StubRouter {}
    impl Router<()> for StubRouter {
        fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<()>) {
            todo!()
        }
    }

    async fn endpoint(_: Request<()>) -> tide::Result {
        todo!()
    }

    #[test]
    fn should_build_basic_route() {
        let mut router = StubRouter {};

        router.register(
            root()
                .method(Method::Get, endpoint)
                .method(Method::Post, endpoint),
        );
    }

    #[test]
    fn should_build_nested_route() {
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
}
