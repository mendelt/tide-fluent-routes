//! The reverse router returns routes by their name.

use crate::HashMap;

/// Stores a list of routes by name
#[derive(Debug)]
pub struct ReverseRouter {
    routes: HashMap<String, String>,
}

impl ReverseRouter {
    /// Insert a named route
    pub fn insert(&mut self, name: &str, route: &str) {
        self.routes.insert(name.to_string(), route.to_string());
    }

    /// Resolve a named route
    pub fn resolve(&self, name: &str, _params: Params) -> String {
        let route = self.routes[name].clone();

        // todo: replace params with values

        route
    }

    /// Construct a named routes list
    pub fn new() -> Self {
        ReverseRouter {
            routes: HashMap::new(),
        }
    }
}

/// Parameters for insertion in paths
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Params(HashMap<String, String>);

impl Params {
    /// Create new params
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a parameter
    pub fn insert(&mut self, param: String, value: String) {
        self.0.insert(param, value);
    }
}