use std::collections::HashMap;
use tide::{Endpoint, http::Method};

pub trait Router<State: Clone + Send + Sync + 'static> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>);

    fn route(&mut self) -> RouteBuilder<State> {
        todo!()
    }
}

impl<State: Clone + Send + Sync + 'static> Router<State> for tide::Server<State> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>) {
        self.at(path).method(method, endpoint);
    }
}

pub struct RouteBuilder<State> {
    _branches: HashMap<String, RouteBuilder<State>>,
    _endpoints: HashMap<Method, Box<dyn Endpoint<State>>>,
}

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> {
    pub fn at(self, _path: &str, _subroute: impl Fn(RouteBuilder<State>) -> RouteBuilder<State>) -> RouteBuilder<State> {
        todo!()
    }

    pub fn method(self, _method: Method, _endpoint: impl Endpoint<State>) -> RouteBuilder<State> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct StubRouter {}
    impl Router<()> for StubRouter {
        fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<()>) {
            todo!()
        }
    }

    #[test]
    fn should_build_basic_route() {
        let router = StubRouter {};

        router.route()
            .method(Method::Get, |_| todo!())
            .method(Method::Post, |_| todo!());
    }

    #[test]
    fn should_build_nested_route() {
        let router = StubRouter {};

        router.route()
            .method(Method::Get, |_| todo!())
            .method(Method::Post, |_| todo!());
            .at("api/v1", |route| route
                .method(Method::Get, |_| todo!())
                .method(Method::Post, |_| todo!())
            )
            .at("api/v2", |route| route
                .method(Method::Get, |_| todo!())
                .method(Method::Post, |_| todo!())
            );
    }
}
