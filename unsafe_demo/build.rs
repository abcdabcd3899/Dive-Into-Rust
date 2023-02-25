extern crate cc;
fn main() {
    println!("cargo:rerun-if-changed=src/triple.cpp");
    cc::Build::new()
        .file("src/test.cpp")
        .cpp(true)
        .compile("libtest.a");
}