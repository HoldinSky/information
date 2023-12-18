use crate::{
    types::{CodeType, EncodingSettings},
    utils::{clear, get_file, logic, pause, terminal::get_line_from_user},
};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};
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

                if let Err(err) = perform_action_by_option(res) {
                    println!("{}", err);
                }
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

fn perform_action_by_option(option: MenuOption) -> Result<(), Error> {
    clear();
    match option {
        MenuOption::StatsByHand => Ok(logic::calculate_user_input_stats()),
        MenuOption::StatsFile => Ok(logic::calculate_file_stats()),
        MenuOption::EncodeFile => {
            let settings = encoding_prerequisites()?;
            Ok(logic::encode_file(settings)?)
        }
        MenuOption::DecodeFile => Ok(logic::decode_file(None)?),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Cannot process the {:?} option", option),
        )),
    }
}

fn encoding_prerequisites() -> Result<EncodingSettings, Error> {
    let file_info = match get_file() {
        Ok(f) => f,
        Err(err) => return Err(Error::new(ErrorKind::NotFound, err)),
    };

    let code_type = loop {
        clear();
        println!("Choose encoding type");
        if let Some(ct) = choose_encoding_type() {
            break ct;
        }
    };

    let hamming_code_length = choose_hamming_code_length()?;

    Ok(EncodingSettings {
        code_type,
        file_info,
        hamming_code_length,
    })
}

fn choose_encoding_type() -> Option<CodeType> {
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

fn choose_hamming_code_length() -> Result<Option<u8>, Error> {
    if !ask_use_hamming_code()? {
        return Ok(None);
    };

    loop {
        clear();
        println!("Input size for hamming code (7 - 255)");
        let input = get_line_from_user();
        let input = input.trim();

        match input.parse::<u8>() {
            Ok(ham_code_lenght) => {
                if ham_code_lenght >= 7 {
                    return Ok(Some(ham_code_lenght));
                } else {
                    pause("Code length must be in range of 7 - 255. Press any key...")
                }
            }
            Err(err) => pause(format!("{}. Press any key...", err).as_str()),
        }
    }
}

fn ask_use_hamming_code() -> Result<bool, Error> {
    loop {
        println!("Use hamming codes to compress data? (y/n)");

        let ans = get_line_from_user().to_lowercase();
        let ans = ans.trim();

        if ans != "y" && ans != "n" {
            println!("Failed to parse the input.");
        } else {
            return Ok(ans == "y");
        }
    }
}
