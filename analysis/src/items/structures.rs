use super::Origin;
use crate::xml::{self, UnwrapBorrowed};

#[derive(Debug)]
pub struct Structure {
    pub origin: Origin,
    pub name: &'static str,
}

impl Structure {
    pub fn new(origin: Origin, xml: &xml::Structure) -> Structure {
        Structure {
            origin,
            name: xml.name.unwrap_borrowed(),
        }
    }
}
