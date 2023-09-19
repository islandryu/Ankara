pub mod cases;
pub mod common;
pub mod misc;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        ast::{self, Expression, Operator},
        interpreter::{
            environment::Environment,
            evaluator::{EvalOption, Evaluator},
            object::Object,
        },
        lexer::Peekable,
        parser::parse,
        parser::parse_expression,
        precedence::Precedence,
    };

    use super::*;

    fn get_result(str: &str) -> Object {
        let mut env = Environment::new(None);
        let mut lexer = Peekable::new(str);
        let program = parse(&mut lexer).unwrap();
        program
            .eval(Rc::new(RefCell::new(env)), &mut EvalOption::new())
            .unwrap()
    }

    #[test]
    fn test_if_else() {
        assert_eq!(
            get_result("return if (true) { 1; } else { 2; }"),
            Object::Number(1)
        );
        assert_eq!(
            get_result("return if (false) { return 1; } else { return 2; }"),
            Object::Number(2)
        );
    }
}
