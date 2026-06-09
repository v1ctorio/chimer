fn main() {
    cxx_build::bridge("src/main.rs")  // returns a cc::Build
        .file("src/chime-bridge.cc")
        .std("c++17")
        .include("src/include")
        .compile("chimer-dev");

    println!("cargo:rerun-if-changed=src/chime-bridge.cc");
    println!("cargo:rerun-if-changed=include/chime-bridge.h");
}