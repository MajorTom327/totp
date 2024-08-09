use clap::{Parser, Subcommand};

/// TOTP Generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The subcommand to use
    #[command(subcommand)]
    pub command: Commands,

    /// The secret to use to generate the TOTP
    pub secret: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate the next token for the secret
    Generate,
    /// Verify if the token is valid
    Verify { token: String },
}