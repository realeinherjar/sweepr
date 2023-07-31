use clap::Parser;

/// Bitcoin address generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Seed to sweep funds from
    seed: String,

    /// Address to withdraw to
    address: String,

    /// Network to use
    #[arg(short, long, default_value = "mainnet")]
    network: String,

    /// Electrum server to use
    #[arg(short, long, default_value = "ssl://electrum.blockstream.info:60002")]
    server: String,
}
