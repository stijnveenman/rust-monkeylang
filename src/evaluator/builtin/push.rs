use crate::object::Object;

pub fn builtin_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            2
        ));
    };

    let mut iter = args.into_iter();
    let array = iter.next().unwrap();
    let item = iter.next().unwrap();

    match array {
        Object::Array(mut a) => {
            a.push(item);

            Object::Array(a)
        }
        e => Object::Error(format!(
            "argument to `push` must be ARRAY, got {}",
            e.type_str()
        )),
    }
}
