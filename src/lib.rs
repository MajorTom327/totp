pub mod totp;
pub mod cli_args;

use std::process::exit;
use crate::totp::Totp;
use crate::cli_args::{CliArgs, Commands};


pub fn run(args: CliArgs) {
    let totp = Totp::new(args.secret);



    match args.command {
        Commands::Generate => {
            generate_totp(totp)
        },
        Commands::Verify { token} => {
            verify_token(totp, token)
        }
    }

}

pub fn generate_totp(totp: Totp) {
    let code = totp.generate();



    println!("\x1b[90m┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃{}┃", format!("{:^30}", "YOUR TOTP CODE"));
    println!("┃\x1b[34m{:^30}\x1b[90m┃", code);
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛\x1b[0m");
}

pub fn verify_token(totp: Totp, token: String) {
    match totp.verify(&token) {
        Ok(()) => println!("\x1b[32;1mToken is valid\x1b[0m"),
        Err(()) => {
            println!("\x1b[31;1mToken is invalid!\x1b[0m");
            exit(-1);
        },
    }
}