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
    pub fn resolve(&self, name: &str) -> String {
        self.routes[name].clone()
    }

    /// Construct a named routes list
    pub fn new() -> Self {
        ReverseRouter {
            routes: HashMap::new(),
        }
    }
}
