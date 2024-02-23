use crate::object::Object;

pub fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            1
        ));
    };

    match args.into_iter().next().unwrap() {
        Object::String(s) => (s.len() as i64).into(),
        Object::Array(a) => (a.len() as i64).into(),
        e => Object::Error(format!(
            "arguments to `len` not supported, got {}",
            e.type_str()
        )),
    }
}
