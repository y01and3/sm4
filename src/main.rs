pub mod func;
use std::io::{self};
fn main() {
    let mut input = String::new();
    println!("Please input the mode [encode, decode]:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mode = input.trim();
    let mode = mode.to_lowercase();
    let mode = mode.as_str();
    let mode_flag = match mode {
        "encode" => true,
        "decode" => false,
        _ => panic!("Invalid mode!"),
    };

    let mut input = String::new();
    println!("Please input the message (32 hex digits):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let message = input.trim();
    print!("The message is: ");
    print!("{}", message);
    let message = match u128::from_str_radix(message, 16) {
        Ok(message) => message,
        Err(_) => panic!("Invalid message!"),
    };

    let mut input = String::new();
    println!("Please input the key (32 hex digits):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let key = match u128::from_str_radix(input.trim(), 16) {
        Ok(key) => key,
        Err(_) => panic!("Invalid key!"),
    };

    if mode_flag {
        let cipher = func::sm4::enc(message, key);
        println!("The cipher is:");
        println!("{:32x}", cipher);
    } else {
        let plain = func::sm4::dec(message, key);
        println!("The plain is:");
        println!("{:32x}", plain);
    }
}
