#![allow(dead_code)]

extern crate strum;
extern crate strum_macros;

mod algorithms;
mod application;
mod bit_map;
mod tests;
mod types;
mod utils;

fn main() {
    application::start();
}
