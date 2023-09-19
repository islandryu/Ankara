use std::array;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;

use crate::ast::{
    self, ArrayMapValue, Assign, BlockExpression, ElementAccessExpression, Expression, Identifier,
    Program, Statement, WatchDeclaration,
};
use crate::interpreter::environment::Environment;
use crate::interpreter::object::{Function, Object};

use super::assign::EvalAssign;
use super::object::{Array, ArrayElement, BlockReturn, Return};

#[derive(Debug, PartialEq, Clone)]
pub struct EvalOption {
    pub watch: Option<Watch>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Watch {
    pub declaration: Rc<RefCell<WatchDeclaration>>,
    pub env: Rc<RefCell<Environment>>,
}

impl EvalOption {
    pub fn new() -> EvalOption {
        EvalOption { watch: None }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub message: String,
    pub child: Option<Box<Error>>,
}

pub trait Evaluator {
    fn eval(&self, env: Rc<RefCell<Environment>>, option: &mut EvalOption)
        -> Result<Object, Error>;
}

impl Evaluator for Program {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let statements = &self.statements;
        let mut value = Object::None;
        let mut iter = statements.iter();
        let mut option_statement = iter.next();
        while option_statement.is_some() && value == Object::None {
            let statement = option_statement.unwrap();

            value = (*statement).eval(env.clone(), option).unwrap();
            option_statement = iter.next();
        }
        Ok(value)
    }
}

impl Evaluator for Statement {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        match &self {
            Statement::VariableDeclaration(variable_declaration) => {
                match variable_declaration.eval(env, option) {
                    Ok(obj) => match obj {
                        Object::Return(_) => return Ok(obj),
                        Object::BlockReturn(_) => return Ok(obj),

                        _ => return Ok(Object::None),
                    },
                    Err(error) => return Err(error),
                }
            }
            Statement::Expression(expression) => match expression.eval(env, option) {
                Ok(obj) => match obj {
                    Object::Return(_) => return Ok(obj),
                    Object::BlockReturn(_) => return Ok(obj),
                    _ => return Ok(Object::None),
                },
                Err(error) => return Err(error),
            },
            Statement::ReturnStatement(return_statement) => {
                match return_statement.eval(env, option) {
                    Ok(value) => return Ok(Object::Return(Box::new(Return { value: value }))),
                    Err(error) => return Err(error),
                }
            }
            Statement::BlockReturnStatement(block_return) => match block_return.eval(env, option) {
                Ok(value) => {
                    return Ok(Object::BlockReturn(Box::new(BlockReturn { value: value })))
                }
                Err(error) => return Err(error),
            },
            Statement::WatchDeclaration(watch_declaration) => {
                match watch_declaration.eval(env, option) {
                    Ok(value) => return Ok(value),
                    Err(error) => return Err(error),
                }
            }
        }
    }
}

impl Evaluator for crate::ast::VariableDeclaration {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let name = self.name.clone();
        let value = self.value.eval(env.clone(), option)?;
        match value {
            Object::Return(_) => return Ok(value),
            _ => {}
        }
        let mut env_borrowed = (*env).borrow_mut();
        env_borrowed.define(name, value);
        Ok(Object::Null)
    }
}

impl Evaluator for Expression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        match &self {
            Expression::NumberLiteral(integer_literal) => integer_literal.eval(env, option),
            Expression::InfixExpression(infix_expression) => infix_expression.eval(env, option),
            Expression::Identifier(identifier) => identifier.eval(env, option),
            Expression::FunctionLiteral(function_declaration) => {
                function_declaration.eval(env, option)
            }
            Expression::CallExpression(call_expression) => call_expression.eval(env, option),
            Expression::IfExpression(if_expression) => if_expression.eval(env, option),
            Expression::BooleanLiteral(boolean_literal) => boolean_literal.eval(env, option),
            Expression::StringLiteral(string_literal) => string_literal.eval(env, option),
            Expression::ArrayLiteral(array_literal) => array_literal.eval(env, option),
            Expression::ElementAccessExpression(element_access_expression) => {
                element_access_expression.eval(env, option)
            }
            Expression::ForExpression(for_expression) => for_expression.eval(env, option),
            Expression::SwitchExpression(switch_expression) => switch_expression.eval(env, option),
            Expression::Assign(assign) => assign.eval(env, option),
            Expression::BlockExpression(block) => block.eval(env, option),
        }
    }
}

impl Evaluator for crate::ast::NumberLiteral {
    fn eval(
        &self,
        _env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        Ok(Object::Number(self.value))
    }
}

impl Evaluator for crate::ast::InfixExpression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let left = self.left.eval(env.clone(), option)?;
        let right = self.right.eval(env, option)?;
        let operator = self.operator.clone();
        match (left, right) {
            (Object::Number(left_value), Object::Number(right_value)) => match operator {
                crate::ast::Operator::Plus => Ok(Object::Number(left_value + right_value)),
                crate::ast::Operator::Minus => Ok(Object::Number(left_value - right_value)),
                crate::ast::Operator::Asterisk => Ok(Object::Number(left_value * right_value)),
                crate::ast::Operator::Slash => Ok(Object::Number(left_value / right_value)),
                crate::ast::Operator::Percent => Ok(Object::Number(left_value % right_value)),
                crate::ast::Operator::Equal => Ok(Object::Boolean(left_value == right_value)),
                crate::ast::Operator::NotEqual => Ok(Object::Boolean(left_value != right_value)),
                crate::ast::Operator::LessThan => Ok(Object::Boolean(left_value < right_value)),
                crate::ast::Operator::LessThanOrEqual => {
                    Ok(Object::Boolean(left_value <= right_value))
                }
                crate::ast::Operator::GreaterThan => Ok(Object::Boolean(left_value > right_value)),
                crate::ast::Operator::GreaterThanOrEqual => {
                    Ok(Object::Boolean(left_value >= right_value))
                }
                crate::ast::Operator::And => {
                    Ok(Object::Boolean(left_value != 0 && right_value != 0))
                }
                crate::ast::Operator::Or => {
                    Ok(Object::Boolean(left_value != 0 || right_value != 0))
                }
                crate::ast::Operator::Bang => Ok(Object::Boolean(left_value == 0)),
            },
            (Object::StringLiteral(left_value), Object::StringLiteral(right_value)) => {
                match operator {
                    crate::ast::Operator::Plus => {
                        Ok(Object::StringLiteral(left_value + &right_value))
                    }
                    crate::ast::Operator::Equal => Ok(Object::Boolean(left_value == right_value)),
                    crate::ast::Operator::NotEqual => {
                        Ok(Object::Boolean(left_value != right_value))
                    }
                    _ => Err(Error {
                        message: "invalid operator".to_string(),
                        child: None,
                    }),
                }
            }
            (Object::Boolean(left_value), Object::Boolean(right_value)) => match operator {
                crate::ast::Operator::Equal => Ok(Object::Boolean(left_value == right_value)),
                crate::ast::Operator::NotEqual => Ok(Object::Boolean(left_value != right_value)),
                _ => Err(Error {
                    message: "invalid operator".to_string(),
                    child: None,
                }),
            },
            _ => Err(Error {
                message: "invalid operator".to_string(),
                child: None,
            }),
        }
    }
}

impl Evaluator for crate::ast::Identifier {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let cloned_env = env.clone();
        match option.watch {
            Some(ref watch) => {
                let watch_declaration = watch.declaration.clone();
                let watch_env = watch.env.clone();
                let mut borrowed = (*cloned_env).borrow_mut();
                borrowed.set_watch(watch_declaration.clone(), watch_env.clone(), &self.value);
            }
            None => {}
        }
        let value = cloned_env.borrow().get(&self.value);
        match value {
            Some(value) => Ok(value),
            None => Err(Error {
                message: "variable not found ".to_string() + &self.value,
                child: None,
            }),
        }
    }
}

impl Evaluator for crate::ast::FunctionLiteral {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let parameters = self.parameters.clone();
        let body = self.body.clone();
        let function = Object::Function(Function {
            parameters,
            body,
            env: env,
        });
        Ok(function)
    }
}

impl Evaluator for crate::ast::CallExpression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let function = self.left.eval(env.clone(), option)?;
        let arguments = self.arguments.clone();
        match function {
            Object::Function(function) => {
                let mut function_env = Environment::new(Some(function.env.clone()));
                for (index, parameter) in function.parameters.iter().enumerate() {
                    let argument = arguments.get(index).unwrap();
                    let value = argument.eval(env.clone(), option)?;
                    function_env.define(parameter.value.clone(), value);
                }
                let result = function
                    .body
                    .eval(Rc::new(RefCell::new(function_env)), option);
                match result {
                    Ok(Object::Return(return_value)) => Ok(return_value.value),
                    Ok(value) => Ok(value),
                    Err(error) => Err(error),
                }
            }
            Object::BuiltInFunction(buildin) => {
                let mut args = Vec::new();
                for argument in arguments {
                    let value = argument.eval(env.clone(), option)?;
                    args.push(value);
                }
                let function = buildin.function;
                function(args);
                Ok(Object::Null)
            }
            _ => Err(Error {
                message: "not a function".to_string() + &self.left.to_string(),
                child: None,
            }),
        }
    }
}

impl Evaluator for crate::ast::BlockExpression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let statements = &self.statements;
        let mut value = Ok(Object::None);
        let mut iter = statements.iter();
        let mut option_statement = iter.next();
        while option_statement.is_some() {
            let statement = option_statement.unwrap();
            value = (*statement).eval(env.clone(), option);
            if value.is_ok() && value.clone().unwrap().is_return_like() {
                break;
            }
            option_statement = iter.next();
        }
        match value {
            Ok(Object::BlockReturn(block_return)) => Ok(block_return.value),
            Ok(value) => match value {
                Object::None => Ok(Object::None),
                _ => Ok(value),
            },
            Err(_) => value,
        }
    }
}

impl Evaluator for crate::ast::ReturnStatement {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let value = self.value.eval(env, option)?;
        Ok(value)
    }
}

impl Evaluator for crate::ast::IfExpression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let condition = self.condition.eval(env.clone(), option)?;
        if !condition.is_falsey() {
            self.consequence.eval(env.clone(), option)
        } else {
            match self.alternative.clone() {
                Some(alt) => alt.eval(env, option),
                _ => Ok(Object::None),
            }
        }
    }
}

impl Evaluator for crate::ast::BooleanLiteral {
    fn eval(
        &self,
        _env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        Ok(Object::Boolean(self.value))
    }
}

impl Evaluator for crate::ast::StringLiteral {
    fn eval(
        &self,
        _env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        Ok(Object::StringLiteral(self.value.to_string()))
    }
}

impl Evaluator for crate::ast::ArrayLiteral {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let mut elements: Vec<ArrayElement> = Vec::new();
        let mut map_elements: HashMap<String, Object> = HashMap::new();
        for element in &self.elements {
            match element {
                ArrayMapValue::Value(val) => {
                    let value = val.eval(env.clone(), option)?;
                    elements.push(ArrayElement::Object(value));
                }
                ArrayMapValue::MapKeyValue(val) => {
                    let value = val.value.eval(env.clone(), option)?;
                    map_elements.insert(val.key.clone(), value);
                    elements.push(ArrayElement::Key(val.key.clone()));
                }
            }
        }
        Ok(Object::Array(Rc::new(Array {
            elements: RefCell::new(elements),
            map: RefCell::new(map_elements),
        })))
    }
}

impl Evaluator for crate::ast::ElementAccessExpression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let left = self.left.eval(env.clone(), option)?;
        let index = self.index.eval(env, option)?;
        match left {
            Object::Array(array) => match index {
                Object::Number(val) => {
                    let elements = array.elements.borrow();
                    let map = array.map.borrow();
                    let index = val as usize;
                    let element = match elements.get(index) {
                        Some(ArrayElement::Object(val)) => val,
                        Some(ArrayElement::Key(val)) => {
                            let key = val.clone();
                            match map.get(&key) {
                                Some(val) => val,
                                None => {
                                    return Err(Error {
                                        message: "key not found".to_string(),
                                        child: None,
                                    })
                                }
                            }
                        }
                        None => {
                            return Err(Error {
                                message: "index out of bounds".to_string(),
                                child: None,
                            })
                        }
                    };
                    Ok(element.clone())
                }
                Object::StringLiteral(val) => {
                    let key = val.clone();
                    match array.map.borrow().get(&key) {
                        Some(val) => Ok(val.clone()),
                        None => {
                            return Err(Error {
                                message: "key not found".to_string(),
                                child: None,
                            })
                        }
                    }
                }
                _ => {
                    return Err(Error {
                        message: "not a number".to_string() + &self.index.to_string(),
                        child: None,
                    })
                }
            },
            _ => {
                return Err(Error {
                    message: "not an array".to_string() + &self.left.to_string(),
                    child: None,
                })
            }
        }
    }
}

impl Evaluator for crate::ast::BlockReturnStatement {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let value = self.value.eval(env, option)?;
        Ok(value)
    }
}

impl Evaluator for crate::ast::ForExpression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let mut value = Ok(Object::None);
        let mut return_array = Array {
            elements: RefCell::new(Vec::new()),
            map: RefCell::new(HashMap::new()),
        };
        let iter = self.iterable.eval(env.clone(), option);
        let mut obj = match iter {
            Ok(obj) => obj,
            Err(error) => return Err(error),
        };
        let array = match obj {
            Object::Array(array) => array,
            _ => {
                return Err(Error {
                    message: "not an array".to_string(),
                    child: None,
                })
            }
        };
        let elements = array.elements.borrow();
        let mut iter = elements.iter();
        let mut option_array_value = iter.next();

        while option_array_value.is_some() {
            let map = array.map.borrow();
            let array_value = match option_array_value.unwrap() {
                ArrayElement::Object(val) => val,
                ArrayElement::Key(key) => {
                    let key = key.clone();
                    match map.get(&key) {
                        Some(val) => val,
                        None => {
                            return Err(Error {
                                message: "key not found".to_string(),
                                child: None,
                            })
                        }
                    }
                }
            };
            let mut for_env = Environment::new(Some(env.clone()));
            for_env.define(self.variable.value.clone(), array_value.clone());
            value = self.body.eval(Rc::new(RefCell::new(for_env)), option);
            match value {
                Ok(Object::Return(_)) => return value,
                Ok(Object::None) => {}
                Ok(obj) => return Ok(obj),
                Err(error) => return Err(error),
            }
            option_array_value = iter.next();
        }
        Ok(Object::None)
    }
}

impl Evaluator for crate::ast::SwitchExpression {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let value = self.expression.eval(env.clone(), option)?;
        let cases = &self.cases;
        for case in cases {
            let condition = match case.condition.eval(env.clone(), option) {
                Ok(condition) => condition,
                Err(error) => return Err(error),
            };

            if condition.is_equal_to(&value) {
                let body = case.body.eval(env.clone(), option)?;
                match body {
                    Object::Return(_) => return Ok(body),
                    Object::None => {}
                    _ => return Ok(body),
                };
            }
        }
        let default = match &self.default {
            Some(default) => default,
            None => {
                return Ok(Object::None);
            }
        };

        match default.body.eval(env, option) {
            Ok(body) => match body {
                Object::Return(_) => return Ok(body),
                Object::None => return Ok(Object::None),
                _ => return Ok(body),
            },
            Err(error) => return Err(error),
        }
    }
}

impl Evaluator for crate::ast::Assign {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let left = self.left.clone();
        match left {
            Expression::Identifier(identifier) => {
                let value = self.right.eval(env.clone(), option)?;
                identifier.assign(env.clone(), value, option)
            }
            Expression::ElementAccessExpression(element_access_expression) => {
                let value = self.right.eval(env.clone(), option)?;
                element_access_expression.assign(env, value, option)
            }
            _ => Err(Error {
                message: "invalid assignment".to_string(),
                child: None,
            }),
        }
    }
}

impl Evaluator for crate::ast::WatchDeclaration {
    fn eval(
        &self,
        env: Rc<RefCell<Environment>>,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let block = Rc::new(RefCell::new(self.block.clone()));
        let mut option = if env.borrow().get(&self.name).is_some() {
            EvalOption { watch: None }
        } else {
            EvalOption {
                watch: Some(Watch {
                    declaration: Rc::new(RefCell::new(self.clone())),
                    env: env.clone(),
                }),
            }
        };
        let value = block.borrow().eval(env.clone(), &mut option)?;
        (*env).borrow_mut().define(self.name.clone(), value);
        return Ok(Object::None);
    }
}
