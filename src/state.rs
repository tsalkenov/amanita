use std::io::Write;
use std::process::Command;
use std::time::SystemTime;
use std::{env, fs, path::PathBuf};

use psutil::process::Process;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};

use crate::{PROC_DIR, PROC_LOG, STATE_FILE};

pub fn state_dir() -> PathBuf {
    let home = env::var("HOME").unwrap();
    let dir = PathBuf::from(home).join(".amanita");
    if !dir.join(PROC_DIR).exists() {
        fs::create_dir_all(&dir.join(PROC_DIR)).expect("Failed to create state dir for amanita");
    }

    dir
}

pub struct ProcState {
    pub name: String,
    pub command: String,
    pub status: ProcStatus,
    pub start_time: SystemTime,
}

pub enum ProcStatus {
    Running(u32, Process),
    Stopped,
}

impl ProcState {
    pub fn create(name: &str, command: &str) -> Result<Self, std::io::Error> {
        let root = state_dir().join(PROC_DIR).join(name);
        match ProcState::receive(name) {
            Ok(process) => match process.status {
                ProcStatus::Running(_, _) => {
                    log::error!("Process with the same name is still running");
                    std::process::exit(1)
                }
                ProcStatus::Stopped => (),
            },
            Err(_) => {
                fs::create_dir_all(&root)?;
            }
        }

        let log_file = fs::File::create(root.join(PROC_LOG))?;
        let args = shlex::split(command).expect("Invalid command");
        match Command::new(&args[0])
            .args(&args[1..])
            .stdout(log_file)
            .spawn()
        {
            Ok(r) => {
                let pid = r.id();
                let process = Process::new(pid).unwrap();

                let state = ProcState {
                    status: ProcStatus::Running(pid, process),
                    command: command.to_owned(),
                    name: name.to_owned(),
                    start_time: SystemTime::now(),
                };

                Ok(state)
            }
            Err(e) => {
                log::error!("Failed to create process with error: {e}");
                std::process::exit(1)
            }
        }
    }
    pub fn receive(name: &str) -> Result<Self, std::io::Error> {
        let static_state = ProcStatic::read(name)?;
        Ok(ProcState::from(&static_state))
    }
    pub fn save(&self) -> Result<(), std::io::Error> {
        ProcStatic::from(self).write()
    }
}

impl From<&ProcStatic> for ProcState {
    fn from(value: &ProcStatic) -> Self {
        let status = match value.status {
            ProcStaticStatus::Running(pid) => match Process::new(pid) {
                Ok(process) => ProcStatus::Running(pid, process),
                Err(_) => ProcStatus::Stopped,
            },
            ProcStaticStatus::Stopped => ProcStatus::Stopped,
        };
        ProcState {
            name: value.name.clone(),
            command: value.command.clone(),
            start_time: value.start_time,
            status,
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ProcStatic {
    pub name: String,
    pub command: String,
    pub status: ProcStaticStatus,
    #[serde_as(as = "TimestampSeconds")]
    pub start_time: SystemTime,
}

#[derive(Serialize, Deserialize)]
pub enum ProcStaticStatus {
    Running(u32),
    Stopped,
}

impl ProcStatic {
    pub fn read(name: &str) -> Result<Self, std::io::Error> {
        let root = state_dir().join(PROC_DIR).join(name);
        let state_file = fs::read_to_string(root.join(STATE_FILE))?;

        Ok(toml::from_str(&state_file).expect("Failed to deserialize state"))
    }

    pub fn write(&self) -> Result<(), std::io::Error> {
        let root = state_dir()
            .join(PROC_DIR)
            .join(&self.name)
            .join(STATE_FILE);
        let mut state_file = fs::File::create(root)?;

        write!(
            state_file,
            "{}",
            toml::to_string(self).expect("Failed to serialize state")
        )
    }

    pub fn delete(self) -> Result<(), std::io::Error> {
        let root = state_dir().join(PROC_DIR).join(&self.name);
        fs::remove_dir_all(root)
    }
}

impl From<&ProcState> for ProcStatic {
    fn from(value: &ProcState) -> Self {
        let status = match value.status {
            ProcStatus::Running(pid, _) => ProcStaticStatus::Running(pid),
            ProcStatus::Stopped => ProcStaticStatus::Stopped,
        };
        ProcStatic {
            name: value.name.clone(),
            command: value.command.clone(),
            start_time: value.start_time,
            status,
        }
    }
}
