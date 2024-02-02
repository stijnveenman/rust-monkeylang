use crate::object::Object;

pub fn builtin_rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            1
        ));
    };

    match args.into_iter().next().unwrap() {
        Object::Array(a) => {
            if a.is_empty() {
                return Object::Null;
            }
            Object::Array(a[1..].to_vec())
        }
        e => Object::Error(format!(
            "argument to `last` must be ARRAY, got {}",
            e.type_str()
        )),
    }
}
