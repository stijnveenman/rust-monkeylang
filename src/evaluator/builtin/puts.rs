use crate::object::Object;

pub fn builtin_puts(args: Vec<Object>) -> Object {
    for o in args {
        println!("{}", o);
    }

    Object::Null
}
