pub mod constants;

mod file_lock_one_process_only;

pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;

pub use file_lock_one_process_only::FileLockOneProcessOnly;
