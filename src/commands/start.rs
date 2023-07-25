use clap::Args;

use crate::process::Proc;

#[derive(Args)]
pub struct StartArgs {
    /// Process name
    name: String,
    /// Command to run process
    command: String,
    /// Restart process on failure
    #[arg(short, long)]
    restart: bool,
}

impl StartArgs {
    pub fn run(self) -> anyhow::Result<()> {
        match Proc::create(&self.name, &self.command, self.restart) {
            Ok(status) => match status {
                Proc::Running(pid) => {
                    log::info!("Successfully spawned process {}", pid);
                    Ok(())
                },
                Proc::Stopped => {
                    log::info!("Process with name '{}' has finished", self.name);
                    Ok(())
                },
                Proc::NotFound => unreachable!()
            }
            Err(e) => {
                log::error!("Error during process spawning {}", e);
                std::process::exit(1)
            }
        }
    }
}
