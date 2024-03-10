use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Scope {
    Global,
    Local,
    Builtin,
    Free,
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
    counts: Vec<usize>,
    free_symbols: Vec<Vec<Symbol>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            maps: vec![HashMap::new()],
            counts: vec![0],
            free_symbols: vec![vec![]],
        }
    }

    pub fn enclose(&mut self) {
        self.maps.push(HashMap::new());
        self.counts.push(0);
        self.free_symbols.push(vec![]);
    }

    pub fn pop(&mut self) {
        self.maps.pop();
        self.counts.pop();
        self.free_symbols.pop();
    }

    #[cfg(test)]
    fn current(&self) -> &HashMap<String, Symbol> {
        self.maps.last().unwrap()
    }

    pub fn num_locals(&self) -> usize {
        *self.counts.last().unwrap()
    }

    pub fn free_symbols(&self) -> &Vec<Symbol> {
        self.free_symbols.last().unwrap()
    }

    pub fn define(&mut self, name: &str) {
        let scope = match self.maps.len() > 1 {
            true => Scope::Local,
            false => Scope::Global,
        };

        let count = *self.counts.last().unwrap();
        let symbol = Symbol::new(name.into(), scope, count);
        *self.counts.last_mut().unwrap() = count + 1;

        self.maps
            .last_mut()
            .unwrap()
            .insert(name.to_string(), symbol);
    }

    pub fn define_builtin(&mut self, index: usize, name: &str) {
        let symbol = Symbol::new(name.into(), Scope::Builtin, index);
        self.maps.last_mut().unwrap().insert(name.into(), symbol);
    }

    fn do_resolve(&self, name: &str) -> Option<(&Symbol, usize)> {
        for (idx, m) in self.maps.iter().rev().enumerate() {
            if let Some(s) = m.get(name) {
                return Some((s, idx));
            }
        }
        None
    }

    // todo now that resolve has Scope::free logic we need to call resolve of outer.
    // in do_resolve we now loop over the maps themselves, this does not honor free of outer
    // scopes.
    // We need a symbol_table that only does symbol table stuff. and a SymbolTableStack
    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        let Some(s) = self.do_resolve(name) else {
            return None;
        };

        if s.1 == 0 {
            return Some(s.0.clone());
        }

        if matches!(s.0.scope, Scope::Global | Scope::Builtin) {
            return Some(s.0.clone());
        }

        let s = s.0.clone();
        let b = self.define_free(&s).clone();

        Some(b)
    }

    pub fn define_free(&mut self, original: &Symbol) -> &Symbol {
        let name = original.name.to_string();
        self.free_symbols.last_mut().unwrap().push(original.clone());
        println!("define {:?}", self.free_symbols().last().unwrap());

        let s = Symbol::new(
            name.to_string(),
            Scope::Free,
            self.free_symbols.last().unwrap().len() - 1,
        );

        self.maps.last_mut().unwrap().insert(name.to_string(), s);
        self.maps.last().unwrap().get(&original.name).unwrap()
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
        table.free_symbols.last().unwrap(),
        &vec![
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
