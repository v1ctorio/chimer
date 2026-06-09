fn main() {
    include!("../example");
    println!("Hello, world!");
    ffi::foo()
}

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!(<example>);
        fn foo ();
    }
}