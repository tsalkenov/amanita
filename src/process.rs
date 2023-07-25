use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use psutil::process::Process;

use crate::{PROC_DIR, PROC_LOG, PROC_PID, STATE_DIR, STATE_LOG, WATCHER_PID};

pub enum Proc {
    Running(u32),
    Stopped,
    NotFound,
}

impl Proc {
    pub fn get(name: &str) -> io::Result<Self> {
        let dir = state_dir().join(PROC_DIR).join(name);
        if !dir.exists() {
            return Ok(Self::NotFound);
        }
        let pid_content = fs::read_to_string(dir.join(PROC_PID))?;
        let pid = pid_content.parse().expect("Invalid pid file");

        if Process::new(pid).is_ok() {
            Ok(Self::Running(pid))
        } else {
            Ok(Self::Stopped)
        }
    }
    pub fn create(name: &str, command: &str, restart: bool) -> io::Result<Self> {
        let dir = state_dir().join(PROC_DIR).join(name);
        match Proc::get(name)? {
            Proc::Stopped => (),
            Proc::NotFound => fs::create_dir_all(&dir)?,
            Proc::Running(pid) => {
                log::error!("Process is already running. id: {pid}");
                std::process::exit(1)
            }
        }
        let args = shlex::split(command).expect("Invalid command");

        if restart {
            let watcher_log = fs::File::create(state_dir().join(STATE_LOG))?;
            daemonize::Daemonize::new()
                .pid_file(dir.join(WATCHER_PID))
                .stdout(watcher_log)
                .start()
                .expect("Failed to daemonize process");
        }
        loop {
            let log_file = fs::File::create(dir.join(PROC_LOG))?;
            match Command::new(&args[0])
                .args(&args[1..])
                .stdout(log_file)
                .spawn()
            {
                Ok(mut r) => {
                    let pid = r.id();
                    let mut pid_file = fs::File::create(dir.join(PROC_PID))?;
                    write!(&mut pid_file, "{}", pid.to_string())?;
                    if restart {
                        let exit_status = r.wait()?;
                        if exit_status.code().is_none() {
                            return Ok(Proc::Stopped);
                        } else {
                            continue;
                        }
                    }
                    return Ok(Proc::Running(pid));
                }
                Err(e) => {
                    log::error!("Failed to create process with error: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

pub fn state_dir() -> PathBuf {
    let home = env::var("HOME").expect("Failed to find home dir");
    PathBuf::from(home).join(STATE_DIR)
}

pub fn setup_state() -> io::Result<()> {
    let home = env::var("HOME").expect("Failed to find home dir");
    let dir = PathBuf::from(home).join(STATE_DIR).join(PROC_DIR);

    fs::create_dir_all(dir)
}
