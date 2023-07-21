use clap::Args;

use crate::state::{ProcState, ProcStatus, ProcStatic};

#[derive(Args)]
pub struct DeleteArgs {
    name: String
}

impl DeleteArgs {
    pub async fn run(self) -> anyhow::Result<()> {
        match ProcState::receive(&self.name) {
            Ok(process) => match process.status {
                ProcStatus::Running(pid, _) => {
                    log::error!("Cannot delete {} as it's occupying {} pid, kill it first", process.name, pid);
                    std::process::exit(1)
                }
                ProcStatus::Stopped => {
                    ProcStatic::from(&process).delete()?;
                    log::info!("Process folder removed");
                    Ok(())
                }
            }
            Err(_) => {
                log::error!("Process does not exist");
                std::process::exit(1)
            }
        }
    }
}
