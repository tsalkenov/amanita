use clap::Args;

use crate::process::Proc;

#[derive(Args)]
pub struct StartArgs {
    /// Process name
    name: String,
    /// Command to run process
    command: String,
}

impl StartArgs {
    pub async fn run(self) -> anyhow::Result<()> {
        match Proc::create(&self.name, &self.command) {
            Ok(status) => {
                if let Proc::Running(pid) = status {
                    log::info!("Successfully spawned process {}", pid);
                }
                Ok(())
            }
            Err(e) => {
                log::error!("Error during process spawning {}", e);
                std::process::exit(1)
            }
        }
    }
}
