#!/usr/bin/env bash

cargo build --release && cp ./target/release/librust_node_addon_template.so index.node
