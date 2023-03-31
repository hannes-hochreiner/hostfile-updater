use clap::{Parser, Subcommand};
use hostfile_updater::{self};
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
    /// Adds entries (comma-separated)
    Add {
        addresses: String,
        hostnames: String,
    },
    /// Removes entries (comma-separated)
    Remove {
        addresses: String,
        hostnames: String,
    },
}

fn main() -> anyhow::Result<()> {
    let hu = Hu::parse();
    let mut buffer = Vec::new();

    io::stdin().read_to_end(&mut buffer)?;

    let mut hf = hostfile_updater::hostfile::Hostfile::new_from_str(&String::from_utf8(buffer)?)?;

    match &hu.command {
        Commands::Add {
            addresses,
            hostnames,
        } => {
            let hostnames: Vec<&str> = hostnames.split(',').collect();

            for address in addresses.split(',') {
                hf.add(address, &hostnames);
            }
        }
        Commands::Remove {
            addresses,
            hostnames,
        } => {
            let hostnames: Vec<&str> = hostnames.split(',').collect();

            for address in addresses.split(',') {
                hf.remove(address, &hostnames);
            }
        }
    }

    io::stdout().write_all(hf.to_string().as_bytes())?;

    Ok(())
}
