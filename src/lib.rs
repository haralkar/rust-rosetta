// Dummy main library
// It also contains a test module, which checks if all source files are covered by `Cargo.toml`
#![feature(plugin)]

#[allow(dead_code)]
#[cfg(not(test))]
fn main() { }
