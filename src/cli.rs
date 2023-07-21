use clap::{Parser, Subcommand};

use crate::commands::{start::StartArgs, kill::KillArgs, list::ListArgs, delete::DeleteArgs};


#[derive(Parser)]
#[command(name = "Amanita")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start process by giving it name and command
    Start(StartArgs),
    /// Kill process
    Kill(KillArgs),
    /// List all running and stopped processes
    List(ListArgs),
    /// Permanently remove any state conne related to process
    Delete(DeleteArgs)
}
