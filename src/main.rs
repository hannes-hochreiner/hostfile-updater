/// hostfile updater
/// CLI tool to update hostfiles (i.e., add and remove entries).
use clap::{Parser, Subcommand};
use hostfile_updater;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Hu {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds entry
    Add { address: String, hostname: String },
    /// Removes entry
    Remove { address: String, hostname: String },
}

fn main() -> anyhow::Result<()> {
    let hu = Hu::parse();
    let mut buffer = Vec::new();

    io::stdin().read_to_end(&mut buffer)?;

    let mut hf = hostfile_updater::hostfile::Hostfile::new_from_str(&String::from_utf8(buffer)?)?;

    match &hu.command {
        Commands::Add { address, hostname } => {
            hf.add(address, hostname);
        }
        Commands::Remove { address, hostname } => {
            hf.remove(address, hostname);
        }
    }

    io::stdout().write_all(hf.to_string().as_bytes())?;

    Ok(())
}
