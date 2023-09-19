extern crate rand;
use crate::{
    ast::{BlockExpression, BlockReturnStatement, Expression, WatchDeclaration},
    interpreter::object::Object,
};
use core::borrow;
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, path::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Object>,
    pub watch: HashMap<String, Watch>,
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub children: Vec<Rc<RefCell<Environment>>>,
    pub id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Watch {
    pub expressions: Rc<RefCell<WatchDeclaration>>,
    pub env: Rc<RefCell<Environment>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Environment {
        let env = Environment {
            values: HashMap::new(),
            watch: HashMap::new(),
            parent: parent.clone(),
            children: Vec::new(),
            id: rand::random(),
        };
        match parent {
            Some(parent) => {
                (*parent)
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(env.clone())));
            }
            None => {}
        }
        env
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.values.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn assign(env: Rc<RefCell<Environment>>, name: &str, value: Object) -> Option<Object> {
        let mut cloned_env = env.clone();
        let mut borrowed_env = (*cloned_env).borrow_mut();
        match borrowed_env.values.get(name) {
            Some(_) => {
                borrowed_env.values.insert(name.to_string(), value.clone());
                borrowed_env.values.get(name).cloned()
            }
            None => match borrowed_env.parent.clone() {
                Some(parent) => Environment::assign(parent, name, value),
                None => None,
            },
        }
    }

    pub fn set_watch(
        &mut self,
        expressions: Rc<RefCell<WatchDeclaration>>,
        env: Rc<RefCell<Environment>>,
        name: &str,
    ) {
        self.watch
            .insert(name.to_string(), Watch { expressions, env });
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut keys: Vec<&String> = self.values.keys().collect();
        keys.sort();
        for key in keys {
            if let Some(value) = self.values.get(key) {
                result.push_str(&format!("{}: {} \n", key, value));
            }
        }
        for val in &self.children {
            result.push_str("{\n");
            result.push_str(val.borrow().to_string().as_str());
            result.push_str("}\n");
            result.push_str("\n");
        }
        result
    }
}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
