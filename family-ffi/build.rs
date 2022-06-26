use cbindgen::{Builder, Language};

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let deps = vec!["family"];

    // 为了可以导出依赖的crate的定义，with_parse_deps(true)/with_parse_include()是必需的
    Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_parse_deps(true)
        .with_parse_include(deps.as_slice())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("family_ffi.h");

    println!("cargo:rerun-if-changed=\"build.rs:src/*.rs:src/ffi/*.rs\"");
}
