use clap::Args;

use crate::state::ProcState;

#[derive(Args)]
pub struct StartArgs {
    /// Process name
    name: String,
    /// Command to run process
    command: String,
}

impl StartArgs {
    pub async fn run(self) -> anyhow::Result<()> {
        match ProcState::create(&self.name, &self.command) {
            Ok(state) => {
                log::info!("Successfully spawned process");
                state.save()?;

                log::info!("Saved process");
                Ok(())
            }
            Err(e) => {
                log::error!("Error during process spawning {}", e);
                std::process::exit(1)
            }
        }
    }
}
