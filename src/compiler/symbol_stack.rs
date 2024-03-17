use core::panic;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::symbol_table::{Scope, Symbol};

pub struct SymbolTable {
    stack: Vec<Arc<Mutex<SymbolScope>>>,
    current: Arc<Mutex<SymbolScope>>,
}

struct SymbolScope {
    outer: Option<Arc<Mutex<SymbolScope>>>,
    map: HashMap<String, Symbol>,
    count: usize,
    free_symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let current = SymbolScope::new(None);
        SymbolTable {
            stack: vec![current.clone()],
            current,
        }
    }

    pub fn enclose(&mut self) {
        let new = SymbolScope::new(Some(self.current.clone()));

        self.stack.push(new.clone());
        self.current = new
    }

    pub fn pop(&mut self) {
        let Some(prev) = self.current.lock().unwrap().outer.clone() else {
            panic!("Current scope does not have outer scope");
        };
        self.stack.pop();
        self.current = prev.clone();
    }

    pub fn define(&mut self, name: &str) {
        let mut current = self.current.lock().unwrap();
        let scope = match current.outer {
            Some(_) => Scope::Local,
            None => Scope::Global,
        };

        let symbol = Symbol::new(name.into(), scope, current.count);
        current.count += 1;

        current.map.insert(name.into(), symbol);
    }

    pub fn define_builtin(&mut self, index: usize, name: &str) {
        let mut current = self.current.lock().unwrap();
        let symbol = Symbol::new(name.into(), Scope::Builtin, index);

        current.map.insert(name.into(), symbol);
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        let mut current = self.current.lock().unwrap();
        current.resolve(name)
    }
}

impl SymbolScope {
    fn new(outer: Option<Arc<Mutex<SymbolScope>>>) -> Arc<Mutex<SymbolScope>> {
        Arc::new(Mutex::new(SymbolScope {
            outer,
            map: HashMap::new(),
            count: 0,
            free_symbols: vec![],
        }))
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        if let Some(s) = self.map.get(name) {
            return Some(s.clone());
        }

        let Some(outer) = self.outer.clone() else {
            return None;
        };

        let mut outer = outer.lock().unwrap();

        let Some(symbol) = outer.resolve(name) else {
            return None;
        };

        if matches!(symbol.scope, Scope::Global | Scope::Builtin) {
            return Some(symbol.clone());
        }

        Some(self.define_free(&symbol))
    }

    fn define_free(&mut self, original: &Symbol) -> Symbol {
        self.free_symbols.push(original.clone());

        let symbol = Symbol::new(
            original.name.to_string(),
            Scope::Free,
            self.free_symbols.len() - 1,
        );

        self.map.insert(original.name.to_string(), symbol);
        self.map.get(&original.name).unwrap().clone()
    }
}
