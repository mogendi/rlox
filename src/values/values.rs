use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::errors::err::ErrTrait;

use super::{err::ValueErr, func::Func};

#[derive(PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Nil,
    Bool(bool),
    Func(Rc<Func>),
}

impl Value {
    pub fn truthy(&self) -> Result<bool, Box<dyn ErrTrait>> {
        match self {
            Value::Number(val) => return Ok(!(*val == 0.0)),
            Value::String(_) => return Ok(true),
            Value::Nil => return Ok(false),
            Value::Bool(val) => return Ok(*val),
            _ => Err(Box::new(ValueErr::new(
                format!("{} can't be coerced into a boolean value", self),
                format!("{}", self),
            ))),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Value::Number(val) => format!("<Number {}>", val.to_string()),
            Value::Nil => "<nil>".to_string(),
            Value::Bool(val) => match val {
                true => format!("<Boolean {}>", String::from("true")),
                false => format!("<Boolean {}>", String::from("false")),
            },
            Value::String(val) => format!("<String {}>", val.to_owned()),
            Value::Func(func) => format!("<Fun {}>", (*func).name()),
        };

        write!(f, "{}", str)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Value::Number(val) => val.to_string(),
            Value::Nil => String::from("nil"),
            Value::Bool(val) => match val {
                true => String::from("true"),
                false => String::from("false"),
            },
            Value::String(val) => val.to_owned(),
            Value::Func(func) => format!("<Fun {}>", (*func).name()),
        };

        write!(f, "{}", str)
    }
}
