#!/bin/bash -e

cargo build
cargo test
cargo test --features primitive_impls
cargo test --features pointer_impls

cd flat_examples
cargo test
cd ../

cd flat_examples_primitives
cargo test
cd ../

cd flat_examples_pointers
cargo test
cd ../


cargo doc --open --no-deps --features primitive_impls
