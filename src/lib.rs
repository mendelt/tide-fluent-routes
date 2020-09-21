use std::collections::HashMap;
use tide::{Endpoint, http::Method};

pub trait Router<State: Clone + Send + Sync + 'static> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>);

    fn route(&mut self, _route: impl Fn(RouteBuilder<State>) -> RouteBuilder<State>) {
        todo!()
    }
}

impl<State: Clone + Send + Sync + 'static> Router<State> for tide::Server<State> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>) {
        self.at(path).method(method, endpoint);
    }
}

pub struct RouteBuilder<State> {
    _path: String,
    _endpoints: HashMap<Method, Box<dyn Endpoint<State>>>,
}

impl<State: Clone + Send + Sync + 'static> RouteBuilder<State> {
    pub fn at(self, _path: &str, _subroute: impl Fn(RouteBuilder<State>) -> RouteBuilder<State>) -> RouteBuilder<State> {
        todo!()
    }

    pub fn method(self, _path: &str, _method: Method, _endpoint: impl Endpoint<State>) -> RouteBuilder<State> {
        todo!()
    }
}
