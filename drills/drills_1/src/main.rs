use std::mem;
// use std::io::{self, Write};

fn main () {
 /*let path = "output.txt";
  let content = "Hello from Rust!\nThis"*/ 
  println!("u8: {}", mem::size_of::<u8>());
    println!("u64: {}", mem::size_of::<u64>());
    println!("Vec<u8>: {}", mem::size_of::<Vec<u8>>());
    println!("Box<u8>: {}", mem::size_of::<Box<u8>>());
}
