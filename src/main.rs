extern crate strum;
#[macro_use] extern crate strum_macros;

mod application;
mod menu;
mod statistic;
mod file_reader;
mod input;
mod utils;

fn main() {
    application::start();
}


