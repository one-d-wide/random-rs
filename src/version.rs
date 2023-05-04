use super::common::*;

#[derive(Debug, Parser)]
pub struct Subcommand {}

pub fn subcommand(_: Subcommand) -> Result<()> {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Copyright (C) {}", env!("CARGO_PKG_AUTHORS"));
    println!("License GPLv3+: GNU GPL version 3 or later");
    println!();
    println!("{}", env!("CARGO_PKG_REPOSITORY"));
    Ok(())
}
