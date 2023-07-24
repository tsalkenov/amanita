use clap::Args;
use psutil::process::Process;

use crate::process::Proc;

#[derive(Args)]
pub struct KillArgs {
    /// Process name
    name: String,
}

impl KillArgs {
    pub async fn run(self) -> anyhow::Result<()> {
        match Proc::get(&self.name)? {
            Proc::NotFound => {
                log::error!("Process not found");
                std::process::exit(1)
            }
            Proc::Stopped => {
                log::error!("Process is already terminated");
                std::process::exit(1)
            }
            Proc::Running(pid) => {
                Process::new(pid)?.kill()?;
                log::info!("Process killed successfully");
                Ok(())
            }
        }
    }
}
