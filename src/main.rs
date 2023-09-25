mod func;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Encode { hex: String, key: String },
    #[command(arg_required_else_help = true)]
    Decode { hex: String, key: String },
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode { hex, key } => {
            let plain_hex = match u128::from_str_radix(hex.trim(), 16) {
                Ok(plain_hex) => plain_hex,
                Err(_) => panic!("Invalid text!"),
            };
            let key = match u128::from_str_radix(key.trim(), 16) {
                Ok(key) => key,
                Err(_) => panic!("Invalid key!"),
            };
            let cipher = func::sm4::enc(plain_hex, key);
            println!("0x{:032x}", cipher);
        }
        Commands::Decode { hex, key } => {
            let cipher_hex = match u128::from_str_radix(hex.trim(), 16) {
                Ok(cipher_hex) => cipher_hex,
                Err(_) => panic!("Invalid text!"),
            };
            let key = match u128::from_str_radix(key.trim(), 16) {
                Ok(key) => key,
                Err(_) => panic!("Invalid key!"),
            };
            let plain = func::sm4::dec(cipher_hex, key);
            println!("0x{:032x}", plain);
        }
    }
}
