#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
fn main() {
  println!("Hello, world!");
}
