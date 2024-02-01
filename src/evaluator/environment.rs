use std::{collections::HashMap, rc::Rc, sync::Mutex};

use crate::object::Object;

#[derive(Debug)]
pub struct Environment {
    hm: HashMap<String, Object>,
    outer: Option<Rc<Mutex<Environment>>>,
}

impl<'a> Environment {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Rc<Mutex<Environment>> {
        Rc::new(Mutex::new(Environment {
            hm: HashMap::new(),
            outer: None,
        }))
    }

    pub fn get(&self, name: &'a str) -> Option<Object> {
        self.hm
            .get(name)
            .cloned()
            .or_else(|| self.outer.clone().and_then(|m| m.lock().unwrap().get(name)))
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.hm.insert(name, value);
    }
}

pub trait Enclose {
    fn enclose(&self) -> Self;
}

impl Enclose for Rc<Mutex<Environment>> {
    fn enclose(&self) -> Self {
        Rc::new(Mutex::new(Environment {
            hm: HashMap::new(),
            outer: Some(self.clone()),
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::{evaluator::environment::Enclose, object::Object};

    use super::Environment;

    #[test]
    fn test_env() {
        let env = Environment::new();

        env.lock()
            .unwrap()
            .set("test".to_string(), Object::Integer(5));

        assert!(env.lock().unwrap().get("test").is_some())
    }

    #[test]
    fn test_enclose() {
        let env = Environment::new();

        env.lock()
            .unwrap()
            .set("test".to_string(), Object::Integer(5));

        let env = env.enclose();

        assert!(env.lock().unwrap().get("test").is_some())
    }

    #[test]
    fn test_enclose_with_set_after() {
        let env = Environment::new();

        let env = env.enclose();

        env.lock()
            .unwrap()
            .set("test".to_string(), Object::Integer(5));

        assert!(env.lock().unwrap().get("test").is_some())
    }

    #[test]
    fn test_enclose_orignal_env_is_unaffected() {
        let env = Environment::new();

        let env2 = env.enclose();

        env2.lock()
            .unwrap()
            .set("test".to_string(), Object::Integer(5));

        assert!(env.lock().unwrap().get("test").is_none())
    }
}
