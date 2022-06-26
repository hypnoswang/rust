use cc;

fn main() {
    cc::Build::new()
        .include("src/c")
        .file("src/c/family.c")
        .static_flag(true)
        .compile("family");
    println!("cargo:rerun-if-changed=\"build.rs:src/c/*.c:src/c/*.h\"");
}
