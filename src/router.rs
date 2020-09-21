use tide::{Endpoint, http::Method};

pub trait Router<State: Clone + Send + Sync + 'static> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>);
}

impl<State: Clone + Send + Sync + 'static> Router<State> for tide::Server<State> {
    fn register_endpoint(&mut self, path: &str, method: Method, endpoint: impl Endpoint<State>) {
        self.at(path).method(method, endpoint);
    }
}
