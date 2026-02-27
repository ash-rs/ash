use super::Code;
use crate::output::{CodeMap, Destination};
use analysis::item::structure::Structure;
use quote::{format_ident, quote};

impl Code for Structure {
    // TODO(friz64) fully implement.
    fn code(&self) -> CodeMap {
        let name = format_ident!("{}", self.name);
        let code = quote! {
            pub struct #name;
        };

        CodeMap::new(Destination(self.required_by), code)
    }
}
