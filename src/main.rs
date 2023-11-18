#![allow(dead_code)]
extern crate strum;
extern crate strum_macros;

mod application;
mod bit_map;
mod constants;
mod shannon_fano;
mod types;
mod utils;

fn main() {
    application::start();
}
