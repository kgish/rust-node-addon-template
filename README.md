# Rust Node Addon Template

A simple Rust Node addon example template using N-API

## Requirements

For this project template, the following is required:

* [Rust](https://www.rust-lang.org)
* [Node.js](https://nodejs.org)
* [npm](https://www.npmjs.com)
* [nvm](https://github.com/nvm-sh/nvm/blob/master/README.md)

You will also require the clang and llvm-dev libraries:

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

1. Object name
2. Function say_hello()
3. Function add_doubles(x,y)
4. Function send_message(str)
5. Promise fibonacci(n)

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

```text
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

## References

Here are a few relevant links that you might find interesting.

* [N-API Documentation](https://nodejs.org/api/n-api.html#n_api_n_api)
* [nodejs-sys](https://crates.io/crates/nodejs-sys)
* [ABI Stability in Node.js](https://nodejs.org/en/docs/guides/abi-stability)
* [Application Binary Interface (ABI)](https://doc.rust-lang.org/reference/abi.html)
* [Rust and Node.js: A match made in heaven](https://blog.logrocket.com/rust-and-node-js-a-match-made-in-heaven/)
* [Writing fast and safe native Node.js modules with Rust](https://blog.risingstack.com/node-js-native-modules-with-rust/)
* [Using rust modules in JavaScript/Web Development](https://medium.com/@atulanand94/using-rust-modules-for-javascript-web-development-part-i-e6dec27df7b2)
