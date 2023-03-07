extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/glad.c")
        .static_flag(true)
        .compile("glad");
}