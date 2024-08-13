use anyhow::Result;
use std::{fs, io::Write};

use crate::models::config::Config;

pub fn write_config(config: Config) -> Result<()> {
    let dir = format!("{}/run-it", dirs::config_dir().unwrap().to_str().unwrap());
    let config_file = format!("{}/config.toml", dir);
    let config_str = toml::to_string(&config)?;

    let mut file = fs::File::create(config_file)?;
    file.write_all(config_str.as_bytes())?;

    Ok(())
}
