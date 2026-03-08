
use anyhow::Result;
use crate::{keystore, output};

pub fn run(json: bool) -> Result<()> {
    let wallets = keystore::list_wallets()?;
    output::print_result(&wallets, json);
    Ok(())
}
