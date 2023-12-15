use crate::{
    types::CodeType,
    utils::{clear, logic, pause, terminal::get_line_from_user},
};
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug, Copy, Clone, EnumString)]
enum MenuOption {
    StatsFile,
    StatsByHand,
    EncodeFile,
    DecodeFile,
    Exit,
}

pub fn menu() {
    let option_map = HashMap::from([
        (1, MenuOption::StatsFile),
        (2, MenuOption::StatsByHand),
        (3, MenuOption::EncodeFile),
        (4, MenuOption::DecodeFile),
        (5, MenuOption::Exit),
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
    println!("4. Decode file");
    println!("5. Exit");
}

fn print_code_types() {
    println!("1. Shannon Fano");
    println!("2. Huffman");
}

fn parse_option_from_str<T: Copy>(map: &HashMap<u8, T>, opt: &str) -> Result<T, String> {
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
        MenuOption::EncodeFile => {
            if let Some(ct) = choose_encoding_type() {
                clear();
                match logic::encode_file(None, ct) {
                    Ok(_) => (),
                    Err(err) => println!("{}", err),
                }
            }
        }
        MenuOption::DecodeFile => match logic::shannon_fano_decode_file(None) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        },
        _ => println!("Cannot process the {:?} option", option),
    };
}

fn choose_encoding_type() -> Option<CodeType> {
    println!("Choose encoding type");

    clear();
    print_code_types();

    let option =
        String::from_utf8(get_line_from_user().into_bytes()).unwrap_or("error".to_string());

    match parse_option_from_str(
        &HashMap::from([(1_u8, CodeType::Huffman), (2_u8, CodeType::ShannonFano)]),
        option.trim(),
    ) {
        Ok(code_type) => Some(code_type),
        Err(message) => {
            println!("{}. Press any key...", message);
            None
        }
    }
}
