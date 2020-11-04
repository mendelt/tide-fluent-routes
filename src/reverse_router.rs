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
    pub fn insert<P: ToString, V: ToString>(&mut self, param: P, value: V) {
        self.0.insert(param.to_string(), value.to_string());
    }
}

/// Construct parameters for the reverse router
#[macro_export]
macro_rules! params {
    ($( $param:expr => $value:expr ),* ) => {{
        let mut pm: Params = Params::new();
        $(pm.insert($param.to_string(), $value);)*
        pm
    }};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_construct_params() {
        let params = params! {"thing" => 5};

        let mut expected = Params::new();
        expected.insert("thing".to_string(), 5);

        assert_eq!(params, expected);
    }

    #[test]
    fn should_construct_multi_value_params() {
        let params = params! {"thing1" => 5, "thing2" => "another thing"};

        let mut expected = Params::new();
        expected.insert("thing1".to_string(), 5);
        expected.insert("thing2", "another thing");

        assert_eq!(params, expected);
    }
}
