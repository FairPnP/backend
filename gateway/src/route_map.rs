use std::collections::HashMap;

use regex::Regex;

pub struct Route {
    pub base_url: String,
    pub path: String,
    pub methods: Vec<String>,
}

impl Route {
    #[allow(dead_code)]
    pub fn resolve_path(&self, path_params: &HashMap<String, String>) -> String {
        let mut new_path = self.path.clone();
        for (key, value) in path_params {
            new_path = new_path.replace(&format!("{{{}}}", key), value);
        }
        new_path
    }
}

pub struct RouteMap {
    routes: Vec<(Regex, Route)>,
}

impl RouteMap {
    pub fn new() -> Self {
        RouteMap { routes: Vec::new() }
    }

    pub fn add_route(&mut self, base_url: &str, path: &str, methods: Vec<&str>) {
        let re = Regex::new(&format!(
            "^{}$",
            path.replace('{', "(?P<").replace('}', ">[^/]+)")
        ))
        .unwrap();

        let route = Route {
            methods: methods.iter().map(|s| s.to_string()).collect(),
            base_url: base_url.to_string(),
            path: path.to_string(),
        };

        self.routes.push((re, route));
    }

    pub fn find_route(&self, external_path: &str) -> Option<(&Route, HashMap<String, String>)> {
        for (re, route) in &self.routes {
            if let Some(caps) = re.captures(external_path) {
                let mut path_params = HashMap::new();
                re.capture_names().for_each(|key| {
                    if let Some(key) = key {
                        if let Some(value) = caps.name(key) {
                            path_params.insert(key.to_string(), value.as_str().to_string());
                        }
                    }
                });
                return Some((route, path_params));
            }
        }
        None
    }
}
