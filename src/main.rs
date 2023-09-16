use std::io::Write;

enum ConvertCase {
    HexDec,
    HexDecL,
    DecHex,
    DecHexL,
}

fn main() {
    println!(
        "Enter Convert Type: \n
        h2d: Single Byte Hexadecimal to Decimal, \n
        h2dl: Large Group of bytes Hexadecimal To Decimal, \n
        d2h: Single Byte Decimal to Hexadecimal, \n
        d2hl: Large Group of bytes Decimal to Hesadecimal \n"
    );

    let mut user_choice: String = String::new();
    print!("=> ");
    std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut user_choice).unwrap();

    let user_choice = user_choice.trim();

    let convert_case = match user_choice {
        "h2d" => ConvertCase::HexDec,
        "h2dl" => ConvertCase::HexDecL,
        "d2h" => ConvertCase::DecHex,
        "d2hl" => ConvertCase::DecHexL,
        _ => panic!("Invalid Choice"),
    };

    match convert_case {
        // Converting from Decimal to Hexadecimal -> d2h
        ConvertCase::DecHex => {
            println!("You want to convert Decimal to Hexadecimal");

            let mut dec: String = String::new();
            print!("Enter Single Decimal Byte: ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut dec).unwrap();

            let dec: u8 = dec.trim().parse().expect("Failed to parse input");

            let hex = format!("0x{:X}", dec);

            println!("Decimal: {} => Hexadecimal: {}", dec, hex);
        }

        // Converting A group of Decimals to Hexadecimal -> d2hl
        ConvertCase::DecHexL => {
            println!("You want to convert a list of Decimals to Hexadecimal. Enter Decimals seperated by whitespace");

            let mut user_dec_string: String = String::new();

            print!("=> ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut user_dec_string).unwrap();

            let dec_vec: Vec<String> = user_dec_string
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            let mut final_convert_vec: Vec<String> = Vec::new();

            for dec in dec_vec {
                let converted_dec: u8 = dec.trim().parse().expect("Failed to Parse");
                let hex: String = format!("0x{:X}", converted_dec);

                final_convert_vec.push(hex);
            }

            println!("Converted Vec: {:?}", final_convert_vec);
        }

        // Converting Hexadecimal To Decimal -> h2d
        ConvertCase::HexDec => {
            println!("You want to conver a single hexadecimal to decimal example: (0xFF or FF) ");

            let mut user_hex: String = String::new();
            print!("Enter Hex => ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut user_hex).unwrap();

            let user_hex: &str = user_hex.trim();

            let final_dec: u32 = u32::from_str_radix(&user_hex[2..], 16).unwrap();

            println!("Converted: {} => {:?}", user_hex, final_dec);
        }

        // Convert a group Hexadecimals to Decimals -> h2dl
        ConvertCase::HexDecL => {
            println!("You want to convert a list of Hexadecimal to Decimal. Enter Hexaecimals seperated by whitespace");

            let mut user_hex_string: String = String::new();

            print!("")
        }
    }
}
