// Ignore code style errors comming from c-binding name convention
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Note that we need to make this function `extern "C"`
pub extern "C" fn rust_callback(value: u64) {
    println!("Welcome back in Rust! Value={}", value);    
}

fn main() {    

    println!("Hello, world from Rust!");

    unsafe {
        // Call helloWorld function written in C
        helloWorld(55);

        // Call power function written in C with callback function written in Rust
        power(2, 10, Some(rust_callback));
    }
}
