use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = 1;
    if a == 1 {
        println!("Hello, ");
        return;
    }
    println!("world!")
}
