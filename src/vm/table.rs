use std::{collections::HashMap, fmt::Display};

use crate::instructions::values::values::Value;

pub struct Table {
    vars: HashMap<String, Value>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            vars: HashMap::new(),
        }
    }

    pub fn add(&mut self, identifier: String, value: Value) {
        self.vars.insert(identifier, value);
    }

    pub fn resolve(&self, identifier: &String) -> Option<Value> {
        if self.vars.contains_key(identifier) {
            return Some(self.vars.get(identifier).unwrap().clone());
        }
        None
    }

    pub fn override_(&mut self, identifier: String, value: Value) -> Option<Value> {
        if self.vars.contains_key(&identifier) {
            return Some(self.vars.insert(identifier, value).unwrap().clone());
        }
        None
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str = str + "{\n";
        for (key, value) in &self.vars {
            str = str + &format!("  \"{}\": {}\n", key, value);
        }
        str = str + "}";
        write!(f, "{}", str)
    }
}
