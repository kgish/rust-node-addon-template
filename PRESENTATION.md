# Presentation Notes

Who am I?

* Have 35+ years in the wonderful world of software development
* Grew up on Assembly and C/C++.

What is Rust?

Rust is a systems programming language which is safe, concurrent and fast.

No GC or runtime overhead.

Low-level OS system calls: embedded device drivers, IoT, operating system design.

https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/README.md

Interoperability.

Thread safe

High performance

Foreign function interface (FFI) is a mechanism by which a program written in one programming language can call 
routines or make use of services written in another.

https://doc.rust-lang.org/nomicon/ffi.html

Functions that are marked extern are made compatible with C code during compilation. They may be called from C code 
with any parameter values. The exact syntax is extern "ABI" where ABI is a calling convention and depends on the 
target platform. The default one is C which corresponds to a standard C calling convention on the target platform.

```
// export a C-compatible function
#[no_mangle]
unsafe extern "C" fn double_input(input: i32) -> i32 {
    input * 2
}
```

https://youtu.be/DnT-LUQgc7s?t=1752

Native bindings to the nodejs' n-api: https://crates.io/crates/nodejs-sys

Start by just rewriting certain performance critical modules in Rust.

Neon allows you to plug Rust into a Node.js app.

Freelancer available for (remote) work.

Neon (https://github.com/neon-bindings/neon) which makes FFI to Rust from Node easy, safe and seamless.

Why use Rust?

* Computational demand
* Performance is predictable
* Low-level access GPIO and GPU
* Cheaper hardware
