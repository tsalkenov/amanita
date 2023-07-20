use amanita::cli::{Cli, Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let cli = Cli::parse();


    match cli.command {
        Commands::Start(args) => args.run().await,
        Commands::Kill(args) => args.run().await,
        Commands::List(args) => args.run().await
    }
}
