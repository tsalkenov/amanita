use std::time::SystemTime;

use psutil::process::Process;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};

use crate::state::{ProcState, ProcStatus};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ProcSchema {
    pub name: String,
    pub command: String,
    pub status: ProcSchemaStatus,
    #[serde_as(as = "TimestampSeconds")]
    pub start_time: SystemTime,
}

#[derive(Serialize, Deserialize)]
pub enum ProcSchemaStatus {
    Running(u32),
    Stopped,
}

impl ProcSchema {
    pub fn update(self) -> ProcState {
        let status = match self.status {
            ProcSchemaStatus::Running(pid) => match Process::new(pid) {
                Ok(process) => ProcStatus::Running(pid, process),
                Err(_) => ProcStatus::Stopped,
            },
            ProcSchemaStatus::Stopped => ProcStatus::Stopped,
        };
        ProcState {
            name: self.name,
            command: self.command,
            start_time: self.start_time,
            status,
        }
    }
}

impl From<&ProcState> for ProcSchema {
    fn from(value: &ProcState) -> Self {
        let status = match value.status {
            ProcStatus::Running(pid, _) => ProcSchemaStatus::Running(pid),
            ProcStatus::Stopped => ProcSchemaStatus::Stopped,
        };
        ProcSchema {
            name: value.name.clone(),
            command: value.command.clone(),
            start_time: value.start_time.clone(),
            status
        }
    }
}
