use crate::object::Object;

pub fn builtin_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            1
        ));
    };

    match args.into_iter().next().unwrap() {
        Object::Array(a) => a.into_iter().next().unwrap_or(Object::Null),
        e => Object::Error(format!(
            "argument to `first` must be ARRAY, got {}",
            e.type_str()
        )),
    }
}
