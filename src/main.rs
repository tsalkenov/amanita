use amanita::{cli::{Cli, Commands}, process::setup_state};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let cli = Cli::parse();

    if let Err(err) = setup_state() {
        log::error!("Error setting up state direcotires. {err}");
        std::process::exit(1)
    }

    match cli.command {
        Commands::Start(args) => args.run(),
        Commands::Kill(args) => args.run(),
        Commands::List(args) => args.run(),
        Commands::Delete(args) => args.run()
    }
}
