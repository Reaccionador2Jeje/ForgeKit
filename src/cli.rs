use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "forgekit",
    version,
    about = "Universal System Toolkit by Atdev Dev"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display operating system and hardware information
    Info,

    /// Check basic system health indicators
    Health,

    /// List storage devices and their capacity
    Storage,

    /// Display memory usage
    Memory,

    /// Run all available read-only diagnostics
    Diagnose,

    /// Display ForgeKit version
    Version,
}
