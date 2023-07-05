//! The purpose of this module is to test whether a Rust test module can be split into standalone file.

#[cfg(test)]
mod tests;

#[allow(dead_code)]
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}