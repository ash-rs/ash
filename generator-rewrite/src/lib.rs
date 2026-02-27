use analysis::Analysis;
use std::{io, path::Path};

mod item;
mod output;
mod util;

pub fn generate(analysis: &Analysis, output_path: impl AsRef<Path>) -> io::Result<()> {
    let codemap = item::build_items_codemap(analysis.items());
    codemap.write(output_path)
}
