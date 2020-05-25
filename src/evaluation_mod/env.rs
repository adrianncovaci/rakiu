use crate::evaluation_mod::evaluate::Object;
use std::collections::HashMap;

pub struct Env {
    pub env: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            env: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: Object) {
        self.env.insert(key, val);
    }

    pub fn get(&mut self, key: &str) -> Option<Object> {
        self.env.get(key).map(|val| val.clone())
    }

    pub fn get_by_ref(&mut self, key: &str) -> Option<&Object> {
        self.env.get(key)
    }
}
