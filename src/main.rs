use clap::Parser;

mod bip39;
mod cli;
mod network;

use self::cli::Args;

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
