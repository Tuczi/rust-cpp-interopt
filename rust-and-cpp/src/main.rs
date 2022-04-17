use autocxx::prelude::*;
use autocxx::subclass::*;

autocxx::include_cpp! {
    #include "example2.hpp" // Relative to whatever we specify in `build.rs`
    generate!("example::Power")
    safety!(unsafe_ffi)
    subclass!("example::Callback", RustCallback)
}

#[is_subclass(superclass("example::Callback"))]
#[derive(Default)]
pub struct RustCallback;

impl ffi::example::Callback_methods for RustCallback {
    fn onResult(&self, value: autocxx::c_long) {
        let value: i64 = value.into();
        println!("Welcome back in Rust! Value={}", value);
    }
}


fn main() {
    println!("Hello, world from Rust!");

    let mut pow = ffi::example::Power::new(2.into()).within_unique_ptr();
    pow.pin_mut().helloWorld(6.into());

    let rust_callback = RustCallback::default_rust_owned();
    pow.pin_mut().power(10.into(), rust_callback.as_ref().borrow().as_ref());
}
