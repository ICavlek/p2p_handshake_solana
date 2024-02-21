use clap::{command, Parser};

/// Arguments structure used to collect necessary information from the user
/// for the Solana Client. It is based on clap crate.
#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
pub struct Arguments {
    pub uri: String,
    #[arg(long, short, default_value_t = 200, help = "connection timeout")]
    pub timeout: u64,
}
