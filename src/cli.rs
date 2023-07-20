use clap::{Parser, Subcommand};

use crate::commands::{start::StartArgs, kill::KillArgs, list::ListArgs};


#[derive(Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Start(StartArgs),
    Kill(KillArgs),
    List(ListArgs)
}
