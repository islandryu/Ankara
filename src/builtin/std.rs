use crate::interpreter::object::Object;

pub fn print(vec: Vec<Object>) -> Object {
    if vec.len() != 1 {
        panic!("wrong number of arguments. got={}, want=1", vec.len());
    }
    let text = match &vec[0] {
        Object::Number(value) => value.to_string(),
        Object::Boolean(value) => value.to_string(),
        obj => obj.to_string(),
    };

    println!("{}", text);
    Object::Null
}
