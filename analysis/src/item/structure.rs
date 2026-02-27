use crate::{item::RequiredBy, xml};

#[derive(Debug)]
pub struct Structure {
    pub required_by: RequiredBy,
    pub name: &'static str,
}

impl Structure {
    pub fn new(required_by: RequiredBy, xml: &xml::Structure) -> Structure {
        Structure {
            required_by,
            name: xml.name,
        }
    }
}
