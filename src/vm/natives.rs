use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::values::{func::Native, values::Value};

use super::table::Table;

pub fn load_natives(global: Rc<RefCell<Table>>) {
    // add `clock`
    (*global).borrow_mut().add(
        "clock".to_string(),
        Value::Native(Rc::new(Native::new(
            "clock".to_string(),
            0,
            Box::new(|stack| {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                (*stack)
                    .borrow_mut()
                    .push(Value::Number(since_the_epoch.as_millis() as f64));
                Ok(())
            }),
        ))),
    );
}
