use clap::Parser;
use totp::{run};
use totp::cli_args::CliArgs;

fn main() {
    let args = CliArgs::parse();

    run(args);
}
