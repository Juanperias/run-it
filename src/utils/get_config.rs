use anyhow::Result;
use std::{fs, io::Write};

use crate::models::config::Config;

pub fn get_config() -> Result<Config> {
    let dir = dirs::config_dir().unwrap().join("run-it");
    let config_file = dir.join("config.toml");

    if let Ok(config_str) = fs::read_to_string(config_file.clone()) {
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    } else {
        println!(
            "Can't read {}, using the default config",
            config_file.to_str().unwrap()
        );
        let config = Config {
            ..Default::default()
        };
        let config_str = toml::to_string(&config)?;
        println!(
            "Writing the default config on {}",
            config_file.to_str().unwrap()
        );
        fs::create_dir(dir)?;
        let mut file = fs::File::create(config_file)?;
        file.write_all(config_str.as_bytes())?;

        Ok(config)
    }
}
