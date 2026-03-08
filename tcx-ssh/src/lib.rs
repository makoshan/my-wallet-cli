pub mod mnemonic;
pub mod ssh_keys;

use core::result;

pub type Result<T> = result::Result<T, anyhow::Error>;

pub mod ssh {
    pub use crate::mnemonic::MnemonicHandler;
    pub use crate::ssh_keys::SshKeyGenerator;
}
