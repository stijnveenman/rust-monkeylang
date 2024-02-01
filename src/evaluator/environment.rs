use std::{collections::HashMap, rc::Rc, sync::Mutex};

use crate::object::Object;

#[derive(Debug)]
pub struct Environment {
    hm: HashMap<String, Object>,
}

impl<'a> Environment {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Rc<Mutex<Environment>> {
        Rc::new(Mutex::new(Environment { hm: HashMap::new() }))
    }

    pub fn get(&self, name: &'a str) -> Option<Object> {
        self.hm.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.hm.insert(name, value);
    }
}
