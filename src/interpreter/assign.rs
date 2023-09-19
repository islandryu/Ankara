use core::borrow;
use std::rc::Rc;
use std::{borrow::BorrowMut, cell::RefCell};

use crate::ast::{ElementAccessExpression, Identifier};

use super::evaluator::EvalOption;
use super::{
    environment::Environment,
    evaluator::{Error, Evaluator},
    object::{ArrayElement, Object},
};

pub trait EvalAssign {
    fn assign(
        &self,
        env: Rc<RefCell<Environment>>,
        value: Object,
        option: &mut EvalOption,
    ) -> Result<Object, Error>;
}

impl EvalAssign for ElementAccessExpression {
    fn assign(
        &self,
        env: Rc<RefCell<Environment>>,
        value: Object,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let left = self.left.eval(env.clone(), option);
        let index = self.index.eval(env, option);

        let array = match left {
            Ok(Object::Array(array)) => array.clone(),
            _ => {
                return Err(Error {
                    message: format!("{} is not an array", left.unwrap()),
                    child: None,
                })
            }
        };

        match index {
            Ok(Object::Number(index)) => {
                let index = index as usize;
                let mut elements = array.elements.borrow_mut();
                if index < elements.len() {
                    elements[index] = ArrayElement::Object(value.clone());
                } else {
                    return Err(Error {
                        message: format!("index out of range: {}", index),
                        child: None,
                    });
                }
            }
            Ok(Object::StringLiteral(index)) => {
                array.map.borrow_mut().insert(index, value.clone());
            }
            _ => {
                return Err(Error {
                    message: format!("{} is not a valid index", index.unwrap()),
                    child: None,
                })
            }
        }

        return Ok(value);
    }
}

impl EvalAssign for Identifier {
    fn assign(
        &self,
        env: Rc<RefCell<Environment>>,
        value: Object,
        option: &mut EvalOption,
    ) -> Result<Object, Error> {
        let name = self.value.clone();
        let ret = value.clone();
        Environment::assign(env.clone(), &name, value);
        let borrowed_env = (*env).borrow();
        let watch = match borrowed_env.watch.get(&name) {
            Some(watch) => watch,
            None => return Ok(ret),
        };
        let mut watch_env = watch.env.clone();
        if env == watch_env {
            watch_env = env.clone();
        }
        let expression = watch.expressions.clone();
        drop(borrowed_env);
        expression.borrow().eval(watch_env, option);
        Ok(ret)
    }
}
