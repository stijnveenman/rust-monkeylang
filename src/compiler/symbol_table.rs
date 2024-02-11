use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub enum Scope {
    Global,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Symbol {
    pub name: String,
    pub scope: Scope,
    pub index: usize,
}

#[derive(Debug)]
pub struct SymbolTable {
    map: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            map: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str) -> &Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: Scope::Global,
            index: self.map.len(),
        };

        self.map.insert(name.to_string(), symbol);

        self.resolve(name).unwrap()
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.map.get(name)
    }
}

#[test]
fn test_define() {
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

    let mut global = SymbolTable::new();

    global.define("a");
    assert_eq!(global.map["a"], expected["a"]);

    global.define("b");
    assert_eq!(global.map["b"], expected["b"]);
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
