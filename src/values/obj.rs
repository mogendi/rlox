use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

use super::values::Value;

pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
{}
<Class {}>
{}
",
            "-".repeat(self.name.len() + 5),
            self.name,
            "-".repeat(self.name.len() + 5),
        )
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Class {}>", self.name)
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

pub struct Instance {
    class: Rc<Class>,
    fields: RefCell<HashMap<String, Value>>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Instance {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn set_prop(&self, name: String, value: Value) {
        self.fields.borrow_mut().insert(name, value);
    }

    pub fn get_prop(&self, name: String) -> Option<Value> {
        if self.fields.borrow().contains_key(&name) {
            return Some(self.fields.borrow().get(&name).unwrap().clone());
        }
        None
    }

    pub fn name(&self) -> String {
        self.class.name.clone()
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Instance of {}>", self.class.name)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Class {}>", self.class.name)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.class.name == other.class.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.class.name != other.class.name
    }
}
