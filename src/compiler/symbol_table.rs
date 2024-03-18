use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::current,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Scope {
    Global,
    Local,
    Builtin,
    Free,
    Function,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: Scope,
    pub index: usize,
}

impl Symbol {
    pub fn new(name: String, scope: Scope, index: usize) -> Symbol {
        Symbol { name, scope, index }
    }
}

pub struct SymbolTable {
    stack: Vec<Arc<Mutex<SymbolScope>>>,
    pub current: Arc<Mutex<SymbolScope>>,
}

pub struct SymbolScope {
    outer: Option<Arc<Mutex<SymbolScope>>>,
    map: HashMap<String, Symbol>,
    pub count: usize,
    pub free_symbols: Vec<Symbol>,
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

    pub fn define_function_name(&mut self, name: &str) {
        let mut current = self.current.lock().unwrap();
        let symbol = Symbol::new(name.into(), Scope::Function, 0);

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

#[test]
fn test_define() {
    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("c", Symbol::new("c".into(), Scope::Local, 0));
    expected.insert("d", Symbol::new("d".into(), Scope::Local, 1));
    expected.insert("e", Symbol::new("e".into(), Scope::Local, 0));
    expected.insert("f", Symbol::new("f".into(), Scope::Local, 1));

    let mut global = SymbolTable::new();

    global.define("a");
    assert_eq!(global.current.lock().unwrap().map["a"], expected["a"]);

    global.define("b");
    assert_eq!(global.current.lock().unwrap().map["b"], expected["b"]);

    global.enclose();

    global.define("c");
    assert_eq!(global.current.lock().unwrap().map["c"], expected["c"]);

    global.define("d");
    assert_eq!(global.current.lock().unwrap().map["d"], expected["d"]);

    global.enclose();

    global.define("e");
    assert_eq!(global.current.lock().unwrap().map["e"], expected["e"]);

    global.define("f");
    assert_eq!(global.current.lock().unwrap().map["f"], expected["f"]);
}

#[test]
fn test_resolve_global() {
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");

    let mut expected = HashMap::new();
    expected.insert(
        "a",
        Symbol {
            name: "a".into(),
            scope: Scope::Global,
            index: 0,
        },
    );
    expected.insert(
        "b",
        Symbol {
            name: "b".into(),
            scope: Scope::Global,
            index: 1,
        },
    );

    for item in expected {
        let result = global.resolve(item.0);

        assert_eq!(result, Some(item.1))
    }
}

#[test]
fn test_resolve_local() {
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");

    global.enclose();
    global.define("c");
    global.define("d");

    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("c", Symbol::new("c".into(), Scope::Local, 0));
    expected.insert("d", Symbol::new("d".into(), Scope::Local, 1));

    for item in expected {
        let result = global.resolve(item.0);

        assert_eq!(result, Some(item.1))
    }
}

#[test]
fn test_resolve_nested_local() {
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");

    global.enclose();
    global.define("c");
    global.define("d");

    global.enclose();
    global.define("e");
    global.define("f");

    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("e", Symbol::new("e".into(), Scope::Local, 0));
    expected.insert("f", Symbol::new("f".into(), Scope::Local, 1));

    for item in expected {
        let result = global.resolve(item.0);

        assert_eq!(result, Some(item.1))
    }

    global.pop();

    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("c", Symbol::new("c".into(), Scope::Local, 0));
    expected.insert("d", Symbol::new("d".into(), Scope::Local, 1));

    for item in expected {
        let result = global.resolve(item.0);

        assert_eq!(result, Some(item.1))
    }
}

#[test]
fn test_builtin_scope() {
    let mut global = SymbolTable::new();
    global.define_builtin(0, "a");
    global.define_builtin(1, "b");
    global.define_builtin(2, "c");
    global.define_builtin(3, "d");

    global.enclose();
    global.enclose();

    assert_eq!(
        global.resolve("a"),
        Some(Symbol::new("a".into(), Scope::Builtin, 0))
    );
    assert_eq!(
        global.resolve("b"),
        Some(Symbol::new("b".into(), Scope::Builtin, 1))
    );
    assert_eq!(
        global.resolve("c"),
        Some(Symbol::new("c".into(), Scope::Builtin, 2))
    );
    assert_eq!(
        global.resolve("d"),
        Some(Symbol::new("d".into(), Scope::Builtin, 3))
    );
}

#[test]
fn test_resolve_free() {
    let mut table = SymbolTable::new();

    table.define("a");
    table.define("b");

    table.enclose();

    table.define("c");
    table.define("d");

    table.enclose();

    table.define("e");
    table.define("f");

    //begin assert

    assert_eq!(
        table.resolve("a").unwrap(),
        Symbol::new("a".into(), Scope::Global, 0)
    );
    assert_eq!(
        table.resolve("b").unwrap(),
        Symbol::new("b".into(), Scope::Global, 1)
    );

    assert_eq!(
        table.resolve("c").unwrap(),
        Symbol::new("c".into(), Scope::Free, 0)
    );
    assert_eq!(
        table.resolve("d").unwrap(),
        Symbol::new("d".into(), Scope::Free, 1)
    );

    assert_eq!(
        table.resolve("e").unwrap(),
        Symbol::new("e".into(), Scope::Local, 0)
    );
    assert_eq!(
        table.resolve("f").unwrap(),
        Symbol::new("f".into(), Scope::Local, 1)
    );

    assert_eq!(
        table.current.lock().unwrap().free_symbols,
        vec![
            Symbol::new("c".into(), Scope::Local, 0),
            Symbol::new("d".into(), Scope::Local, 1)
        ]
    );

    table.pop();

    assert_eq!(
        table.resolve("a").unwrap(),
        Symbol::new("a".into(), Scope::Global, 0)
    );
    assert_eq!(
        table.resolve("b").unwrap(),
        Symbol::new("b".into(), Scope::Global, 1)
    );

    assert_eq!(
        table.resolve("c").unwrap(),
        Symbol::new("c".into(), Scope::Local, 0)
    );
    assert_eq!(
        table.resolve("d").unwrap(),
        Symbol::new("d".into(), Scope::Local, 1)
    );
}

#[test]
fn test_resolve_unresolved_free() {
    let mut table = SymbolTable::new();
    table.define("a");
    table.enclose();

    table.define("c");
    table.enclose();

    table.define("e");
    table.define("f");

    assert_eq!(
        table.resolve("a").unwrap(),
        Symbol::new("a".into(), Scope::Global, 0)
    );
    assert_eq!(
        table.resolve("c").unwrap(),
        Symbol::new("c".into(), Scope::Free, 0)
    );
    assert_eq!(
        table.resolve("e").unwrap(),
        Symbol::new("e".into(), Scope::Local, 0)
    );
    assert_eq!(
        table.resolve("f").unwrap(),
        Symbol::new("f".into(), Scope::Local, 1)
    );

    assert!(table.resolve("b").is_none());
    assert!(table.resolve("d").is_none());
}

#[test]
fn test_define_and_resolve_function_name() {
    let mut table = SymbolTable::new();

    table.define_function_name("a");

    assert_eq!(
        table.resolve("a"),
        Some(Symbol::new("a".into(), Scope::Function, 0))
    );
}

#[test]
fn test_shadowing_function_name() {
    let mut table = SymbolTable::new();

    table.define_function_name("a");

    table.define("a");

    assert_eq!(
        table.resolve("a"),
        Some(Symbol::new("a".into(), Scope::Global, 0))
    );
}
