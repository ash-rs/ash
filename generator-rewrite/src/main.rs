use analysis::Analysis;

fn main() {
    tracing_subscriber::fmt::init();
    let analysis = Analysis::new("generator/Vulkan-Headers");
    generator_rewrite::generate(&analysis, "ash-rewrite/src/generated").unwrap();
}
