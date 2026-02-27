use crate::output::CodeMap;
use analysis::item::Items;
use tracing::debug;

mod structure;

pub trait Code {
    fn code(&self) -> CodeMap;
}

impl CodeMap {
    pub fn extend_from_items<'a, C: Code + 'a>(
        &mut self,
        item_iter: impl IntoIterator<Item = &'a C>,
    ) {
        for item in item_iter {
            self.extend(item.code());
        }
    }
}

pub fn build_items_codemap(items: &Items) -> CodeMap {
    debug!("building codemap");
    let mut codemap = CodeMap::default();

    debug!("generating structures code");
    codemap.extend_from_items(items.structures.values());

    codemap
}
