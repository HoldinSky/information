extern crate strum;
extern crate strum_macros;

use std::env;
use std::os::unix::prelude::OsStringExt;

mod application;
mod menu;
mod statistic;
mod file_reader;
mod input;
mod utils;
mod terminal;

fn main() {
    application::start();
}

fn print_entries_of_current_dir() {
    println!("Entries in {:?}:", env::current_dir().unwrap());
    for file in terminal::list_files().unwrap() {
        println!("{}", String::from_utf8(file.into_vec()).unwrap());
    }
}

fn test_directory_change() {
    print_entries_of_current_dir();

    terminal::change_dir("..");
    print_entries_of_current_dir();

    terminal::change_dir("..");
    print_entries_of_current_dir();

    terminal::change_dir("/opfwe");
    print_entries_of_current_dir();
}
