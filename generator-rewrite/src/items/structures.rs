use super::Code;
use crate::CodeMap;
use analysis::items::structures::Structure;
use quote::{format_ident, quote};

impl Code for Structure {
    // TODO(friz64) fully implement.
    fn code(&self) -> CodeMap {
        let name = format_ident!("{}", self.name);

        CodeMap::new(
            self.origin,
            quote! {
                pub struct #name;
            },
        )
    }
}
