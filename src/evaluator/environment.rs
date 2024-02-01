use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    hm: HashMap<String, Object>,
}

impl<'a> Environment {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Environment {
        Environment { hm: HashMap::new() }
    }

    pub fn get(&self, name: &'a str) -> Option<Object> {
        self.hm.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.hm.insert(name, value);
    }
}
