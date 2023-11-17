use std::collections::HashMap;

use strum_macros::EnumString;

use crate::utils::{clear, logic, pause, terminal::get_line_from_user};

#[derive(Debug, Clone, EnumString)]
enum MenuOption {
    StatsFile,
    StatsByHand,
    EncodeFile,
    Exit,
}

pub fn menu() {
    let option_map = HashMap::from([
        (1, MenuOption::StatsFile),
        (2, MenuOption::StatsByHand),
        (3, MenuOption::EncodeFile),
        (4, MenuOption::Exit),
    ]);

    loop {
        clear();
        print_options();

        let option =
            String::from_utf8(get_line_from_user().into_bytes()).unwrap_or("error".to_string());

        match parse_option_from_str(&option_map, option.trim()) {
            Ok(res) => {
                // catch Exit option
                match res {
                    MenuOption::Exit => break,
                    _ => (),
                };

                perform_action_by_option(res)
            }
            Err(message) => {
                pause(format!("{}. Press any key...", message).as_str());
                continue;
            }
        };

        pause("Press any key...");
    }
}

fn print_options() {
    println!("1. Calculate statistics of file");
    println!("2. Calculate statistics of terminal input");
    println!("3. Encode file");
    println!("4. Exit");
}

fn parse_option_from_str(map: &HashMap<u8, MenuOption>, opt: &str) -> Result<MenuOption, String> {
    let option = match opt.parse::<u8>() {
        Ok(val) => val,
        Err(_) => return Err(String::from("Option must be a digit")),
    };

    match map.get(&option).cloned() {
        Some(val) => Ok(val),
        None => Err(String::from("There is no such option")),
    }
}

fn perform_action_by_option(option: MenuOption) {
    clear();
    match option {
        MenuOption::StatsByHand => logic::calculate_user_input_stats(),
        MenuOption::StatsFile => logic::calculate_file_stats(),
        MenuOption::EncodeFile => match logic::encode_file(None) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        },
        _ => println!("Cannot process the {:?} option", option),
    };
}
