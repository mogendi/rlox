use std::fmt::{Debug, Display};

pub trait ErrTraitBase {
    fn raise(&self);
}

pub trait ErrTrait: ErrTraitBase + Debug + Display {}
impl<T> ErrTrait for T where T: ErrTraitBase + Debug + Display {}
