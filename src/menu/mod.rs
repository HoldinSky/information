use std::collections::HashMap;

use strum_macros::EnumString;

use crate::input;
use crate::terminal::take_line;
use crate::utils::{clear, pause};

#[derive(Debug, Clone, EnumString)]
enum MenuOption {
    File,
    ByHand,
    Exit,
}

pub fn menu() {
    let option_map = HashMap::from([(1, MenuOption::File), (2, MenuOption::ByHand), (3, MenuOption::Exit)]);

    loop {
        clear();
        print_options();

        let option = String::from_utf8(take_line()).unwrap_or("error".to_string());

        match parse_option_from_str(&option_map, option.trim()) {
            Ok(res) => {
                match res {
                    MenuOption::Exit => break,
                    _ => (),
                };

                process_menu_option(res)
            },
            Err(message) => {
                pause(format!("{}. Press any key...", message).as_str());
                continue;
            }
        };

        pause("Press any key...");
    }
}

fn print_options() {
    println!("1. Input from file");
    println!("2. Input by hand in terminal");
    println!("3. Exit");
}

fn parse_option_from_str(map: &HashMap<i32, MenuOption>, opt: &str) -> Result<MenuOption, String> {
    let option = match opt.parse::<i32>() {
        Ok(val) => val,
        Err(_) => { return Err(String::from("Option must be a digit")) }
    };

    match map.get(&option).cloned() {
        Some(val) => Ok(val),
        None => Err(String::from("There is no such option"))
    }
}

fn process_menu_option(option: MenuOption) {
    clear();
    match option {
        MenuOption::ByHand => input::process_terminal_input(),
        MenuOption::File => input::process_file_input(),
        _ => println!("Cannot process the {:?} option", option),
    };
}