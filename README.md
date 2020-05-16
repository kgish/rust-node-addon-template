# Rust Node Addon Template

A simple Rust Node addon example template using N-API

## Introduction

This is part of the presentation that I gave at a recent [Berlin Rust Meetup](https://www.meetup.com/Rust-Berlin/).

## Requirements

For this project template, the following is required:

* [Rust](https://www.rust-lang.org)
* [Node.js](https://nodejs.org)
* [npm](https://www.npmjs.com)
* [nvm](https://github.com/nvm-sh/nvm/blob/master/README.md)

In order to build the nodejs-sys crate (belwo) you need libclang since bindings are being generated at build-time by 
bindgen. Therefore, you will require the clang and llvm-dev libraries:

```bash
$ sudo apt-get install llvm-dev clang
```

Ensure that you have the correct Node version installed:

```bash
$ nvm install 12.16.3
$ nvm use 12.16.3
```

## Installation and setup

Download the project from github:

```bash
$ git clone https://github.com/kgish/rust-node-addon-template.git
$ cd rust-node-addon-template 
$ chmod +x *.sh
```

## Build

```bash
$ ./build.sh
```

## Run

```bash
$ ./run.sh [n]
```

where optional n = 1 - 5 in order to run only given example:

1. Function sayHello() => void
2. Function sendMessage(str) => void
3. Function addNumbers(x,y) => number
4. Function getUser() => user
5. Promise fibonacci(n) => number

## From Scratch

Rather than cloning the github project, you might prefer to build everything from scratch. 

Initialize the node project and create the `package.json` configuration file by running:

```bash
$ npm init
```

Create a Rust package with a library target `src/lib.rs` by executing.

```bash
$ cargo init --lib --crate-type=cdylib
```

The `--crate-type=cdylib` flag produces a dynamic system library which is used when compiling a dynamic library to be loaded from another language. This output type will create the relevant file type for other operating systems, the *.so file for Linux.

The project directory should look like this:

```text
├── Cargo.toml
├── package.json
└── src
    └── lib.rs
```

Add the `nodejs-sys` crate (native bindings to the nodejs' n-api) as a dependency to the `Cargo.toml` file:

Cargo.toml
```text
[package]
name = "rust-node-addon-template"
version = "0.1.0"
authors = ["Kiffin Gish <kiffin.gish@planet.nl>"]
edition = "2018"

[lib]
crate-type=["cdylib"]

[dependencies]
nodejs-sys = "0.3.0"
```

Create the following files in the root directory:

build.sh
```bash
#!/usr/bin/env bash

cargo build --release && cp ./target/release/librust_node_addon_template.so index.node
```

run.sh
```bash
#!/usr/bin/env bash

node ./index.js $1
```

index.js
```javascript
let addon = require('./index.node');

// Add stuff you want to use here
```

Run the demo.

```bash
$ chmod +x *.sh
$ ./build.sh
$ ./run.sh
```

## FFI bindings

`std::ffi::CString` - A type representing an owned, C-compatible, nul-terminated string with no nul bytes in the middle.
This type serves the purpose of being able to safely generate a C-compatible string from a Rust byte slice or vector. An
instance of this type is a static guarantee that the underlying bytes contain no interior 0 bytes ("nul characters") and
that the final byte is 0 ("nul terminator").

See: [Struct std::ffi::CString](https://doc.rust-lang.org/std/ffi/struct.CString.html)

`std::ffi::c_void` - Equivalent to C's void type when used as a pointer.
In essence, *const c_void is equivalent to C's const void* and *mut c_void is equivalent to
C's void*. That said, this is not the same as C's void return type, which is Rust's () type.

See: [Enum std::ffi::c_void](https://doc.rust-lang.org/std/ffi/enum.c_void.html)

`#[no_mangle]` - this attribute turns off Rust’s name mangling, so that it is easier to link to. The Rust compiler 
mangles symbol names differently than native code linkers expect and therefore needs to be told NOT to mangle any 
functions exported to the outside world.

## extern "C"

By default, any function you write in Rust will use the Rust ABI. Instead, when building outwards facing FFI APIs we 
need to tell the compiler to use the system ABI.

See the chapter [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) in the Rust Book.

## Raw bindings to the Node.js API

Here is an overview of the types, enums and functions used from [nodejs_sys](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys).

Types:
  * [napi_async_work](https://docs.rs/node-api-sys/0.3.0/node_api_sys/type.napi_async_work.html)
  * [napi_callback_info](https://docs.rs/node-api-sys/0.3.0/node_api_sys/type.napi_callback_info.html)
  * [napi_deferred](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/type.napi_deferred.html)
  * [napi_env](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/type.napi_env.html)
  * [napi_value](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/type.napi_value.html)
  
Enum:
  * [napi_status](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/enum.napi_status.html)
  
Functions:
  * [napi_create_async_work](https://docs.rs/node-api-sys/0.3.0/node_api_sys/fn.napi_create_async_work.html)
  * [napi_create_double](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_double.html)
  * [napi_create_error](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_error.html)
  * [napi_create_function](https://docs.rs/nodejs-sys/0.2.0/nodejs_sys/fn.napi_create_function.html)
  * [napi_create_int64](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_int64.html)
  * [napi_create_object](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_object.html)
  * [napi_create_promise](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_promise.html)
  * [napi_create_string_utf8](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_string_utf8.html)
  * [napi_create_uint32](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_create_uint32.html)
  * [napi_delete_async_work](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_delete_async_work.html)
  * [napi_get_cb_info](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_cb_info.html)
  * [napi_get_undefined](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_undefined.html)
  * [napi_get_value_double](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_double.html)
  * [napi_get_value_int64](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_int64.html)
  * [napi_get_value_string_utf8](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_get_value_string_utf8.html)
  * [napi_queue_async_work](https://docs.rs/node-api-sys/0.3.0/node_api_sys/fn.napi_queue_async_work.html)
  * [napi_reject_deferred](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_reject_deferred.html)
  * [napi_resolve_deferred](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_resolve_deferred.html)
  * [napi_set_named_property](https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_set_named_property.html)

## Neon

Rust bindings for writing safe and fast native Node.js modules.

### Installation

```bash
$ npm install -g neon-cli
$ neon new my-project
$ cd my-project
$ npm install
```
The directory tree should look something like this.

```
.
├── lib
│   └── index.js
├── native
│   ├── build.rs
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── package.json
└── README.md
```

lib/index.js
```javascript
const addon = require('../native/index.node');

// Add stuff you want to use here
```
native/build.rs
```rust
extern crate neon_build;

fn main() {
    neon_build::setup(); // must be called in build.rs

    // add project-specific build logic here...
}
```

native/Cargo.html
```
[package]
name = "my-project"
version = "0.1.0"
authors = ["Kiffin Gish <kiffin.gish@planet.nl>"]
license = "MIT"
build = "build.rs"
edition = "2018"
exclude = ["artifacts.json", "index.node"]

[lib]
name = "my_project"
crate-type = ["cdylib"]

[build-dependencies]
neon-build = "0.4.0"

[dependencies]
neon = "0.4.0"
```

native/src/lib.rs
```
use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
```



## References

Here are a few relevant links that you might find interesting.

* [N-API Documentation](https://nodejs.org/api/n-api.html#n_api_n_api)
* [nodejs-sys](https://crates.io/crates/nodejs-sys)
* [ABI Stability in Node.js](https://nodejs.org/en/docs/guides/abi-stability)
* [Application Binary Interface (ABI)](https://doc.rust-lang.org/reference/abi.html)
* [Rust and Node.js: A match made in heaven](https://blog.logrocket.com/rust-and-node-js-a-match-made-in-heaven/)
* [Writing fast and safe native Node.js modules with Rust](https://blog.risingstack.com/node-js-native-modules-with-rust/)
* [Using rust modules in JavaScript/Web Development](https://medium.com/@atulanand94/using-rust-modules-for-javascript-web-development-part-i-e6dec27df7b2)
* [Neon](https://neon-bindings.com)
* [An introduction to Neon](https://www.youtube.com/watch?v=yj2nD9hB3D0)
* [Writing Node.js Modules in Rust](https://www.youtube.com/watch?v=5Cbjk8w9mEM)
* [High Performance Apps with JavaScript and Rust, It's Easier Than You Think](https://www.youtube.com/watch?v=Pfbw4YPrwf4)

