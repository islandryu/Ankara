use crate::ast::{BlockReturnStatement, Expression};
use crate::{ast, interpreter::environment::Environment};
use std::ops::Deref;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};
#[derive(PartialEq, Clone)]
pub enum Object {
    Number(i32),
    Boolean(bool),
    Function(Function),
    BuiltInFunction(BuiltInFunction),
    StringLiteral(String),
    Array(Rc<Array>),
    Return(Box<Return>),
    BlockReturn(Box<BlockReturn>),
    None,
    Null,
    Void,
}

impl Object {
    pub fn is_number(&self) -> bool {
        match self {
            Object::Number(_) => true,
            _ => false,
        }
    }
    pub fn unwrap_number(&self) -> i32 {
        match self {
            Object::Number(value) => *value,
            _ => panic!("unwrap_number called on non-number"),
        }
    }
    pub fn is_falsey(&self) -> bool {
        match self {
            Object::Boolean(value) => !value,
            Object::Null => true,
            Object::Void => true,
            Object::None => true,
            Object::Number(value) => *value == 0,
            _ => false,
        }
    }
    pub fn is_return(&self) -> bool {
        match self {
            Object::Return(_) => true,
            _ => false,
        }
    }
    pub fn is_return_like(&self) -> bool {
        match self {
            Object::Return(_) => true,
            Object::BlockReturn(_) => true,
            _ => false,
        }
    }
    pub fn unwrap_block_return(&self) -> Object {
        match self {
            Object::BlockReturn(block_return) => block_return.value.clone(),
            _ => self.clone(),
        }
    }
    pub fn unwrap_return(&self) -> Object {
        match self {
            Object::Return(return_value) => return_value.value.clone(),
            _ => self.clone(),
        }
    }
    pub fn is_equal_to(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Number(left), Object::Number(right)) => left == right,
            (Object::Boolean(left), Object::Boolean(right)) => left == right,
            (Object::StringLiteral(left), Object::StringLiteral(right)) => left == right,
            (Object::Null, Object::Null) => true,
            (Object::Void, Object::Void) => true,
            (Object::None, Object::None) => true,
            _ => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Function(_) => write!(f, "function"),
            Object::BuiltInFunction(_) => write!(f, "builtin function"),
            Object::StringLiteral(value) => write!(f, "{}", value),
            Object::Array(array) => {
                let mut elements = String::new();
                for (i, element) in array.elements.borrow().iter().enumerate() {
                    match element {
                        ArrayElement::Object(object) => {
                            elements.push_str(&format!("{},", object));
                        }
                        ArrayElement::Key(key) => {
                            elements.push_str(&format!("{}:", key));
                            elements
                                .push_str(&format!("{},", array.map.borrow().get(key).unwrap()));
                        }
                    }
                }
                write!(f, "[{}]", elements)
            }
            Object::Null => write!(f, "null"),
            Object::Void => write!(f, "void"),
            Object::None => write!(f, "none"),
            Object::Return(_) => write!(f, "return"),
            Object::BlockReturn(_) => write!(f, "block return"),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Function(_) => write!(f, "function"),
            Object::BuiltInFunction(_) => write!(f, "builtin function"),
            Object::StringLiteral(value) => write!(f, "{}", value),
            Object::Array(array) => {
                let mut elements = String::new();
                for (i, element) in array.elements.borrow().iter().enumerate() {
                    match element {
                        ArrayElement::Object(object) => {
                            elements.push_str(&format!("{},", object));
                        }
                        ArrayElement::Key(key) => {
                            elements.push_str(&format!("{}:", key));
                            elements
                                .push_str(&format!("{},", array.map.borrow().get(key).unwrap()));
                        }
                    }
                }
                write!(f, "[{}]", elements)
            }
            Object::Null => write!(f, "null"),
            Object::Void => write!(f, "void"),
            Object::None => write!(f, "none"),
            Object::Return(_) => write!(f, "return"),
            Object::BlockReturn(_) => write!(f, "block return"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<ast::Identifier>,
    pub body: ast::BlockExpression,
    pub env: Rc<RefCell<Environment>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub function: fn(Vec<Object>) -> Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub elements: RefCell<Vec<ArrayElement>>,
    pub map: RefCell<HashMap<String, Object>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrayElement {
    Object(Object),
    Key(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockReturn {
    pub value: Object,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: Object,
}
