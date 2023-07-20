use std::fs;
use std::io::Write;

use clap::Args;

use crate::{
    schema::ProcSchema,
    state::{state_dir, ProcState},
};

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
                log::info!("Succesfully spawned process");
                let root = state_dir().join(format!("procs/{}", self.name));

                let state_content = ProcSchema::from(&state);

                write!(
                    fs::File::create(root.join("state.toml")).expect("Failed to create state file"),
                    "{}",
                    toml::to_string(&state_content).expect("Failed to deserialize state")
                ).expect("Failed to save state in created state file");

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
