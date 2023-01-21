use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

use super::{
    func::{Func, Method},
    values::Value,
};

pub struct Class {
    name: String,
    methods: Rc<RefCell<HashMap<String, Rc<Func>>>>,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class {
            name,
            methods: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn set_method(&self, method: Func) {
        (*self.methods)
            .borrow_mut()
            .insert(method.name(), Rc::new(method));
    }

    pub fn get_method(&self, name: String) -> Option<Rc<Func>> {
        if (*self.methods).borrow().contains_key(&name) {
            return Some((*self.methods).borrow().get(&name).unwrap().clone());
        }
        None
    }

    pub fn inherit(&self, parent: Rc<Class>) {
        for method in (*(*parent).methods).borrow_mut().iter() {
            let contains_key = self.methods.borrow().contains_key(method.0);
            if !contains_key {
                self.methods
                    .borrow_mut()
                    .insert(method.0.clone(), method.1.clone());
            }
        }
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

    pub fn get_prop(&self, name: String, inst_pointer: Rc<Instance>) -> Option<Value> {
        if self.fields.borrow().contains_key(&name) {
            return Some(self.fields.borrow().get(&name).unwrap().clone());
        }
        match self.class.get_method(name) {
            Some(func) => return Some(Value::Method(Method::new(func.clone(), inst_pointer))),
            None => None,
        }
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
