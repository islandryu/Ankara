use crate::interpreter::{
    environment::Environment,
    object::{BuiltInFunction, Object},
};

use super::std::print;

pub fn get_builtin_environment() -> Environment {
    let mut env = Environment::new(None);
    env.define(
        "print".to_string(),
        Object::BuiltInFunction(BuiltInFunction {
            name: "print".to_string(),
            function: print,
        }),
    );
    env
}
