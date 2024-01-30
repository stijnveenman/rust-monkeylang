use std::collections::HashMap;

use crate::object::Object;

pub struct Environment<'a> {
    hm: HashMap<&'a str, Object>,
}

impl<'a> Environment<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Environment<'a> {
        Environment { hm: HashMap::new() }
    }

    pub fn get(&self, name: &'a str) -> Option<&Object> {
        self.hm.get(name)
    }

    pub fn set(&mut self, name: &'a str, value: Object) {
        self.hm.insert(name, value);
    }
}
