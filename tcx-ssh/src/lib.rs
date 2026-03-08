pub mod ssh_keys;

use core::result;
pub type Result<T> = result::Result<T, anyhow::Error>;
