use analysis::Analysis;
use std::io;
use tracing::debug;

fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    debug!("running analysis");
    let analysis = Analysis::new("generator-rewrite/Vulkan-Headers");
    debug!("running generator");
    generator_rewrite::generate(&analysis, "ash-rewrite/src/generated")
}
