# rust_built_with_C

# Automatically generating the C API for Rust crate

To automatically generate API header for C the `cbindgen` tool can be used.

Link: https://github.com/eqrion/cbindgen

Example usage: cbindgen --crate test-lib --output test.h --lang c