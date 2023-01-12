use crate::errors::err::{ErrTrait, ErrTraitBase};

pub struct RuntimeErr {}

impl ErrTraitBase for RuntimeErr {
    fn raise(&self) {}
}
