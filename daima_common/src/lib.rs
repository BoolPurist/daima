pub mod constants;

mod binary_generation;
mod binary_type_marshalling;
mod file_lock_one_process_only;
pub mod ipc_message;
pub mod message;

mod binary_parsing;

pub type NumberBytesSize = usize;
pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;

pub use file_lock_one_process_only::FileLockOneProcessOnly;
