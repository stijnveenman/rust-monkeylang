use crate::object::Object;

pub fn builtin_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            1
        ));
    };

    match args.into_iter().next().unwrap() {
        Object::Array(a) => a.last().cloned().unwrap_or(Object::Null),
        e => Object::Error(format!(
            "argument to `last` must be ARRAY, got {}",
            e.type_str()
        )),
    }
}
