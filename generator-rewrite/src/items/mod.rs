use crate::CodeMap;
use analysis::Analysis;

mod structures;

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

pub fn build_codemap(analysis: &Analysis) -> CodeMap {
    let mut codemap = CodeMap::default();
    codemap.extend_from_items(analysis.items().structures.values());
    codemap
}
