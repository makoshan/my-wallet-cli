
use anyhow::Result;
use dirs::home_dir;
use std::path::PathBuf;

pub fn wallet_dir() -> PathBuf {
    home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".wallet")
}
pub fn keystore_dir() -> PathBuf { wallet_dir().join("keystores") }
pub fn ensure_dirs() -> Result<()> {
    std::fs::create_dir_all(keystore_dir())?;
    Ok(())
}
