use clap::Args;

use crate::state::{ProcState, ProcStatus};

#[derive(Args)]
pub struct KillArgs {
    /// Process name
    name: String,
}

impl KillArgs {
    pub async fn run(self) -> anyhow::Result<()> {
        let Ok(mut state) = ProcState::receive(&self.name) else {
            log::error!("Process has never been created");
            std::process::exit(1)
        };
        match state.status {
            ProcStatus::Running(_, process) => {
                process.kill()?;
                state.status = ProcStatus::Stopped;
                state.save()?;

                log::info!("Process killed successfully");
                Ok(())
            },
            ProcStatus::Stopped => {
                log::error!("Process is already terminated");
                std::process::exit(1)
            }
        }
    }
}
