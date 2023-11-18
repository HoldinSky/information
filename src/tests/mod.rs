#![cfg(test)]
#![allow(unused_must_use)]

use crate::{
    bit_container::BitContainer,
    utils::{clear, logic::encode_file, pause, print_entries_of_current_dir, terminal},
};
use std::fs::File;

#[test]
fn test_bit_container() {
    let mut cont = BitContainer::new();

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(false);

    assert_eq!(cont.get_bytes(1), [59]);

    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);

    assert_eq!(cont.get_bytes(4), [59, 170]);

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);

    assert_eq!(cont.get_bytes(4), [59, 170, 235]);

    cont.add_bit(true);
    cont.add_bit(true);

    assert_eq!(cont.get_bytes(4), [59, 170, 235]);
}

#[test]
fn test_flush() {
    let mut cont = BitContainer::new();

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(false);

    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);

    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(false);
    cont.add_bit(true);
    cont.add_bit(true);
    cont.add_bit(true);

    cont.add_bit(true);
    cont.add_bit(true);

    let mut file = File::create("/home/nazar/prg/rust/labs/tik/samples/test_bits").unwrap();
    cont.flush_to_file(&mut file).unwrap();
}

#[test]
fn test_encoding() {
    let path = String::from("/home/nazar/prg/rust/labs/tik/samples/en/test_1.txt");
    let file = File::open(&path).unwrap();

    encode_file(Some((file, path)));
}

#[test]
fn test_directory_change() {
    clear();
    print_entries_of_current_dir();

    pause("Press any key...");
    clear();
    terminal::change_dir("..");
    print_entries_of_current_dir();

    pause("Press any key...");
    clear();
    terminal::change_dir("..");
    print_entries_of_current_dir();

    pause("Press any key...");
    clear();
    terminal::change_dir("/opfwe");
    print_entries_of_current_dir();
    pause("Press any key...");
}
