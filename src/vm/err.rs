use crate::errors::err::ErrTraitBase;

pub struct RuntimeErr {}

impl ErrTraitBase for RuntimeErr {
    fn raise(&self) {}
}
