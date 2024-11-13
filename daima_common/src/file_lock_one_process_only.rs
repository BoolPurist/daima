use crate::{constants, AppResult};
use anyhow::bail;
use fslock::LockFile;
use log::info;
use std::path::PathBuf;

pub struct FileLockOneProcessOnly {
    lock: LockFile,
    path: PathBuf,
}

impl Drop for FileLockOneProcessOnly {
    fn drop(&mut self) {
        self.lock
            .unlock()
            .unwrap_or_else(|_| panic!("Should be the only one unlocking file at {:?}", self.path));
        info!("Unlocked file at {:?}", &self.path);
        info!("Delting file at {:?}", &self.path);
        let _ = std::fs::remove_file(self.path.clone());
    }
}

impl FileLockOneProcessOnly {
    pub fn new_for_daemon() -> AppResult<Self> {
        Self::new(constants::APP_DAEMON_NAME)
    }

    pub fn new(app_name: &str) -> AppResult<Self> {
        let full_path = ensure_only_one_running_program(app_name)?;
        let mut locked_file = LockFile::open(&full_path)?;
        let exclusive_access = locked_file.try_lock_with_pid()?;
        if !exclusive_access {
            bail!("No more than process {} is allowed to run", app_name);
        }

        Ok(Self {
            lock: locked_file,
            path: full_path,
        })
    }

    pub fn write_own_pid_to_lock_file(&self) -> AppResult {
        let process_id = std::process::id();
        let path_to_lock_file = &self.path;
        std::fs::write(path_to_lock_file, &process_id.to_string())?;
        info!("Locking running the program to run only once at the same time");
        info!(
            "Locking is done by the locking file at {:?}",
            &path_to_lock_file
        );
        Ok(())
    }
}

fn ensure_only_one_running_program(app_name: &str) -> AppResult<PathBuf> {
    let locked_file_name = format!("lock_file_{}.process", app_name);
    let full_path = get_socket_path().join(PathBuf::from(&locked_file_name));
    Ok(full_path)
}

const PROJECT_PATH: &str = env!("CARGO_PKG_REPOSITORY");

fn get_socket_path() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from(PROJECT_PATH)
    } else {
        std::env::temp_dir()
    }
}
