use generator::write_source_code;
use std::path::Path;

fn main() {
    let headers = "Vulkan-Headers";
    let cwd = std::env::current_dir().unwrap();

    let (base_path, src_dir) = if cwd.ends_with("generator") {
        (Path::new("."), Path::new("../ash-path/src"))
    } else {
        (Path::new("generator"), Path::new("ash-patch/src"))
    };

    let headers = base_path.join(headers);

    write_source_code(&headers, src_dir);
}
