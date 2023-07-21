use std::io::Write;
use std::process::Command;
use std::time::SystemTime;
use std::{env, fs, path::PathBuf};

use psutil::process::Process;

use crate::schema::ProcSchema;
use crate::{PROC_DIR, PROC_LOG, STATE_FILE};

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
    pub fn create(name: &str, command: &str) -> Result<Self, anyhow::Error> {
        let root = state_dir().join(PROC_DIR).join(name);
        fs::create_dir_all(&root)?;
        let log_file =
            fs::File::create(root.join(PROC_LOG)).expect("Failed to create log file");
        if let Ok(old_process) = ProcState::receive(name) {
            if let ProcStatus::Running(_, _) = old_process.status {
                log::error!("Process with the same name is still running");
                std::process::exit(1)
            }
        }

        let args = shlex::split(command).unwrap();
        match Command::new(&args[0])
            .args(&args[1..])
            .stdout(log_file)
            .spawn()
        {
            Ok(r) => {
                let pid = r.id();
                let process = Process::new(pid)?;

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
    pub fn receive(name: &str) -> Result<Self, ()> {
        let root = state_dir().join(PROC_DIR).join(name);
        if !root.join(STATE_FILE).exists() {
            return Err(());
        }

        let state_file = fs::read_to_string(root.join("state.toml")).unwrap();
        let static_state: ProcSchema = toml::from_str(&state_file).unwrap();
        let state = static_state.update();
        state.save_changes();

        Ok(state)
    }
    pub fn save_changes(&self) {
        let root = state_dir().join(PROC_DIR).join(&self.name);
        let state_data = ProcSchema::from(self);
        let serialized_data = toml::to_string(&state_data).expect("Failed to serialize");

        let mut state_file =
            fs::File::create(root.join("state")).expect("Failed to open state file");
        write!(state_file, "{}", serialized_data).expect("Failed to write to state file");
    }
}

pub fn state_dir() -> PathBuf {
    let home = env::var("HOME").unwrap();
    let dir = PathBuf::from(home).join(".amanita");
    if !dir.join(PROC_DIR).exists() {
        fs::create_dir_all(&dir.join("procs")).expect("Failed to create state dir for amanita");
    }

    dir
}
