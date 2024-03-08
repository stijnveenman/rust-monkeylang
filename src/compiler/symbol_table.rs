use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Scope {
    Global,
    Local,
    Builtin,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: Scope,
    pub index: usize,
}

impl Symbol {
    fn new(name: String, scope: Scope, index: usize) -> Symbol {
        Symbol { name, scope, index }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    maps: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            maps: vec![HashMap::new()],
        }
    }

    pub fn enclose(&mut self) {
        self.maps.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.maps.pop();
    }

    fn current(&self) -> &HashMap<String, Symbol> {
        self.maps.last().unwrap()
    }

    pub fn num_locals(&self) -> usize {
        self.current().len()
    }

    pub fn define(&mut self, name: &str) {
        let scope = match self.maps.len() > 1 {
            true => Scope::Local,
            false => Scope::Global,
        };

        let symbol = Symbol::new(name.into(), scope, self.current().len());

        self.maps
            .last_mut()
            .unwrap()
            .insert(name.to_string(), symbol);
    }

    pub fn define_builtin(&mut self, index: usize, name: &str) {
        let symbol = Symbol::new(name.into(), Scope::Builtin, index);
        self.maps.last_mut().unwrap().insert(name.into(), symbol);
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        for m in self.maps.iter().rev() {
            if let Some(s) = m.get(name) {
                return Some(s);
            }
        }
        None
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
    assert_eq!(global.current()["a"], expected["a"]);

    global.define("b");
    assert_eq!(global.current()["b"], expected["b"]);

    global.enclose();

    global.define("c");
    assert_eq!(global.current()["c"], expected["c"]);

    global.define("d");
    assert_eq!(global.current()["d"], expected["d"]);

    global.enclose();

    global.define("e");
    assert_eq!(global.current()["e"], expected["e"]);

    global.define("f");
    assert_eq!(global.current()["f"], expected["f"]);
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

        assert_eq!(result, Some(&item.1))
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

        assert_eq!(result, Some(&item.1))
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

        assert_eq!(result, Some(&item.1))
    }

    global.pop();

    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("c", Symbol::new("c".into(), Scope::Local, 0));
    expected.insert("d", Symbol::new("d".into(), Scope::Local, 1));

    for item in expected {
        let result = global.resolve(item.0);

        assert_eq!(result, Some(&item.1))
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
        Some(&Symbol::new("a".into(), Scope::Builtin, 0))
    );
    assert_eq!(
        global.resolve("b"),
        Some(&Symbol::new("b".into(), Scope::Builtin, 1))
    );
    assert_eq!(
        global.resolve("c"),
        Some(&Symbol::new("c".into(), Scope::Builtin, 2))
    );
    assert_eq!(
        global.resolve("d"),
        Some(&Symbol::new("d".into(), Scope::Builtin, 3))
    );
}
