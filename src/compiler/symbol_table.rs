use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Scope {
    Global,
    Local,
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
    map: HashMap<String, Symbol>,

    outer: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            map: HashMap::new(),
            outer: None,
        }
    }

    pub fn enclose(&mut self) -> SymbolTable {
        SymbolTable {
            map: HashMap::new(),
            outer: Some(Box::new(self.clone())),
        }
    }

    pub fn define(&mut self, name: &str) -> &Symbol {
        let scope = match self.outer {
            Some(_) => Scope::Local,
            None => Scope::Global,
        };

        let symbol = Symbol::new(name.into(), scope, self.map.len());

        self.map.insert(name.to_string(), symbol);

        self.resolve(name).unwrap()
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.map
            .get(name)
            .or_else(|| self.outer.as_ref().and_then(|o| o.resolve(name)))
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
    assert_eq!(global.map["a"], expected["a"]);

    global.define("b");
    assert_eq!(global.map["b"], expected["b"]);

    let mut first = global.enclose();

    first.define("c");
    assert_eq!(first.map["c"], expected["c"]);

    first.define("d");
    assert_eq!(first.map["d"], expected["d"]);

    let mut second = global.enclose();

    second.define("e");
    assert_eq!(second.map["e"], expected["e"]);

    second.define("f");
    assert_eq!(second.map["f"], expected["f"]);
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

    let mut local = global.enclose();
    local.define("c");
    local.define("d");

    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("c", Symbol::new("c".into(), Scope::Local, 0));
    expected.insert("d", Symbol::new("d".into(), Scope::Local, 1));

    for item in expected {
        let result = local.resolve(item.0);

        assert_eq!(result, Some(&item.1))
    }
}

#[test]
fn test_resolve_nested_local() {
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");

    let mut first = global.enclose();
    first.define("c");
    first.define("d");

    let mut second = first.enclose();
    second.define("e");
    second.define("f");

    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("c", Symbol::new("c".into(), Scope::Local, 0));
    expected.insert("d", Symbol::new("d".into(), Scope::Local, 1));

    for item in expected {
        let result = first.resolve(item.0);

        assert_eq!(result, Some(&item.1))
    }

    let mut expected = HashMap::new();
    expected.insert("a", Symbol::new("a".into(), Scope::Global, 0));
    expected.insert("b", Symbol::new("b".into(), Scope::Global, 1));
    expected.insert("e", Symbol::new("e".into(), Scope::Local, 0));
    expected.insert("f", Symbol::new("f".into(), Scope::Local, 1));

    for item in expected {
        let result = second.resolve(item.0);

        assert_eq!(result, Some(&item.1))
    }
}
