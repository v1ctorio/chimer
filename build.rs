
fn main() {
    let chime_sdk_dir = "thirdparty/chime-sdk-signaling-cpp";
    
    cxx_build::bridge("src/main.rs") // returns a cc::Build
        .file("src/chime-bridge.cc")
        .std("c++17")
        .include("src/include")
        .compile("chimer-dev");


    println!("cargo:rustc-link-search=native={}/build", chime_sdk_dir);
    println!("cargo:rustc-link-search=native={}/build/src", chime_sdk_dir);
    println!("cargo:rustc-link-search=native={}/build/_deps/libwebsockets-build/lib", chime_sdk_dir);
    println!("cargo:rustc-link-search=native={}/build/_deps/protobuf-build", chime_sdk_dir);

    println!("cargo:rerun-if-changed=src/chime-bridge.cc");
    println!("cargo:rerun-if-changed=include/chime-bridge.h");
    println!("cargo:rustc-link-lib=static=amazon-chime-signaling-sdk-cpp");
    println!("cargo:rustc-link-lib=static=websockets");
    println!("cargo:rustc-link-lib=static=protobufd");
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
