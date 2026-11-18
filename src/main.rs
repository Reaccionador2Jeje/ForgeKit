mod cli;
mod diagnostics;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    println!("ForgeKit v{}", env!("CARGO_PKG_VERSION"));

    match cli.command {
        Commands::Info => diagnostics::system::show_info(),
        Commands::Health => diagnostics::health::show_health(),
        Commands::Storage => diagnostics::storage::show_storage(),
        Commands::Memory => diagnostics::system::show_memory(),
        Commands::Diagnose => diagnostics::run_diagnostics(),
        Commands::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
    }
}
