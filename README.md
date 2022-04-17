# Rust Cpp Interoperability example

This is an example of 2 simple Rust programs that call C and C++ library back and forth.
For Rust and C++, Rust program is in `rust-and-cpp` and C++ library is in `cpp-lib` (`cpp-lib` static library).
For Rust and C, Rust program is in `rust-and-c` and C library is in `cpp-lib` (`c-lib` static library).

## Building

You might need to install `libclang-dev` first.

Compiling all:
1. go to cpp directory: `cd cpp-lib` 
1. build library using cmake: `cmake --build .` (builds both `c-lib` and `cpp-lib`)
1. go to rust directory: `cd ../rust-and-c` or `cd ../rust-and-cpp`
1. build rust program using carog: `cargo build`
1. run rust program: `cargo run`

Note that C++ and C code is build independently. It means that cpp library could be also provided as `.so` or `.a` by your existing build pipeline without rebuilding everything from scratch in Rust build.

This code is part of my blog article [https://kuczma.dev/articles/rust-and-cpp/](https://kuczma.dev/articles/rust-and-cpp/).
