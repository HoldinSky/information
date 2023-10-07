use std::io::{Read, stdin, stdout, Write};

pub fn take_input() -> Vec<u8> {
    print!("# ");
    stdout().flush().unwrap();

    let mut stdin = stdin();
    let mut buffer = Vec::new();

    stdin.read_to_end(&mut buffer).expect("Could not read input");

    buffer
}

pub fn take_line() -> Vec<u8> {
    print!("# ");
    stdout().flush().unwrap();

    let stdin = stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).expect("Could not read input");

    buffer.into_bytes()
}