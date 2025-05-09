use analysis::Analysis;

fn main() {
    tracing_subscriber::fmt::init();
    let _analysis = Analysis::new("generator/Vulkan-Headers");
    // dbg!(_analysis);
}
