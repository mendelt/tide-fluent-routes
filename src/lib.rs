use std::collections::HashMap;
use tide::{Endpoint, http::Method};

pub trait Router<State: Clone + Send + Sync + 'static> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>);

    fn register(&mut self, routes: RouteBuilder<State>) {
        todo!()
    }
}

impl<State: Clone + Send + Sync + 'static> Router<State> for tide::Server<State> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>) {
        self.at(path).method(method, endpoint);
    }
}

pub fn routes<State>() -> RouteBuilder<State> {
    RouteBuilder {
        _branches: HashMap::new(),
        _endpoints: HashMap::new(),
    }
}

pub struct RouteBuilder<State> {
    _branches: HashMap<String, RouteBuilder<State>>,
    _endpoints: HashMap<Method, Box<dyn Endpoint<State>>>,
}

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> {
    pub fn at(self, _path: &str, _subroutes: RouteBuilder<State>) -> RouteBuilder<State> {
        todo!()
    }

    pub fn method(self, _method: Method, _endpoint: impl Endpoint<State>) -> RouteBuilder<State> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use tide::Request;
    use super::*;

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

        router.register( routes()
            .method(Method::Get, endpoint)
            .method(Method::Post, endpoint)
        );
    }

    #[test]
    fn should_build_nested_route() {
        let mut router = StubRouter {};

        router.register(routes()
            .method(Method::Get, endpoint)
            .method(Method::Post, endpoint)
            .at("api/v1", routes()
                .method(Method::Get, endpoint)
                .method(Method::Post, endpoint)
            )
            .at("api/v2", routes()
                .method(Method::Get, endpoint)
                .method(Method::Post, endpoint)
        ));
    }
}
