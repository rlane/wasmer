use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
}

fn main() {
    println!("Hello, world! root = {}", project_root().display());
}
