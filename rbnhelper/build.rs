extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .file("RBR/IRust.cpp")
        .compile("rbnhelper");
}