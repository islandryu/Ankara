#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        ast::{self, Expression, Operator},
        interpreter::{
            environment::Environment,
            evaluator::Evaluator,
            object::{Array, Object},
        },
        interpreter::{
            evaluator::EvalOption,
            object::{self, Return},
        },
        lexer::Peekable,
        parser::parse,
        parser::parse_expression,
        precedence::Precedence,
    };

    use super::*;

    fn get_result(source_code: &str) -> Object {
        let mut env = Environment::new(None);
        let mut lexer = Peekable::new(source_code);
        let program = parse(&mut lexer).unwrap();
        program
            .eval(Rc::new(RefCell::new(env)), &mut EvalOption::new())
            .unwrap()
    }

    fn get_return_object(obj: Object) -> Object {
        return Object::Return(Box::new(Return { value: obj }));
    }

    #[test]
    fn test_element_access_expression() {
        let val = get_result(
            "\
            let x = [1, 2, 3];
            return x[0];
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(1));
    }

    #[test]
    fn test_for_loop() {
        let val = get_result(
            "\
            let x = [1, 2, 3];
            let last = for (value in x) {
                if (value == 3) {
                    value
                }
            };
            return last;
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(3));
    }

    #[test]
    fn test_switch_expression() {
        let val = get_result(
            "\
            let x = 2;
            let a = switch (x) {
                case 1: {1}
                case 2: {2}
            };
            return a;
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(2));
    }

    #[test]
    fn test_default_case() {
        let val = get_result(
            "\
            let x = 2;
            let a = switch (x) {
                case 1: {1}
                default: {2}
            };
            return a;
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(2));
    }

    #[test]
    fn test_array_map() {
        let val = get_result(
            "\
            let x = [1, 2, 3, myKey: 4];
            return x[\"myKey\"];
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(4));
    }

    #[test]
    fn test_assign() {
        let val = get_result(
            "\
            let x = 1;
            let fnc = fn() {
                x = 2;
            };
            fnc();
            return x;
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(2));
    }

    #[test]
    fn test_assign_element_access() {
        let val = get_result(
            "\
        let x = [1, 2, 3];
        let fnc = fn() {
            x[0] = 2;
        };
        fnc();
        return x[0];
        ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(2));
    }

    #[test]
    fn test_watch() {
        let val = get_result(
            "\
            let x = 1;
            let y = 2;
            watch result = {
                x + y
            };
            x = 2;
            return result;
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(4));
    }

    #[test]
    fn test_block_expression() {
        let val = get_result(
            "\
            let x = 1;
            let y = 2;
            let result = {
                x + y
            };
            return result;
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(3));
    }

    #[test]
    fn test_block_level_return() {
        let val = get_result(
            "\
            let array = [1, 2, 3, 4, 5];
            let val = for(i in array) {
                if(i == 3) {
                    \"i == 3\"
                }
            };
            return val;
            ",
        );
        assert_eq!(
            val.unwrap_return(),
            Object::StringLiteral("i == 3".to_string())
        );
    }

    #[test]
    fn test_function_level_return() {
        let val = get_result(
            "\
            let array = [1, 2, 3, 4, 5];
            let fnc = fn() {
                let val = for(i in array) {
                    if(i == 3) {
                        return \"i == 3\";
                    }
                };
            };
            return fnc();
            ",
        );
        assert_eq!(
            val.unwrap_return(),
            Object::StringLiteral("i == 3".to_string())
        );
    }

    #[test]
    fn test_sample_code1() {
        let val = get_result(
            "\
            let x = 1;
            let a = if (x == 1) {
                1
            } else {
                2
            }; 
            return a;
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(1));
    }

    #[test]
    fn test_sample_code2() {
        let val = get_result(
            "\
            let x = \"hello\";
            let isHello = fn(x) {
                if (x == \"hello\") {
                    return true;
                } else {
                    return false;
                }
            };
            return isHello(x);
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Boolean(true));
    }
    #[test]
    fn test_sample_code3() {
        let val = get_result(
            "\
            let fnc1 = fn() {
                return 1;
            };
            let fnc2 = fn() {
                return fnc1();
            };
            let fnc3 = fn(cb) {
                return cb();
            };\
            return fnc3(fnc2);
            ",
        );
        assert_eq!(val.unwrap_return(), Object::Number(1));
    }

    #[test]
    fn test_sample_code4() {
        let val = get_result(
            "\
                let fnc3 = fn() {
                    {
                        {
                            \"a\"
                        }
                    }
                };

                let fnc3Return = fnc3();
                return fnc3Return;

            ",
        );
        assert_eq!(val.unwrap_return(), Object::StringLiteral("a".to_string()));
    }
}
