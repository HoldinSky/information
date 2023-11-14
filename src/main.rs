#![allow(dead_code)]
extern crate strum;
extern crate strum_macros;

mod application;
mod shannon_fano;
mod utils;

fn main() {
    application::start();
}
