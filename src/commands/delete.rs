use std::fs;

use clap::Args;

use crate::{
    process::{state_dir, Proc},
    PROC_DIR,
};

#[derive(Args)]
pub struct DeleteArgs {
    name: String,
}

impl DeleteArgs {
    pub fn run(self) -> anyhow::Result<()> {
        let dir = state_dir().join(PROC_DIR).join(&self.name);

        match Proc::get(&self.name)? {
            Proc::Stopped => {
                fs::remove_dir_all(dir)?;
            }
            Proc::NotFound => {
                log::error!("Cannot delete non existant process");
                std::process::exit(1)
            }
            Proc::Running(pid) => {
                log::error!("Cannot delete non running process. pid: {pid}");
                std::process::exit(1)
            }
        }

        Ok(())
    }
}
