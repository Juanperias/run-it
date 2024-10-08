use anyhow::{anyhow, Result};
use std::{fs, path::Path};

use crate::commands::{create::create_container, install::install_command};
use crate::models::manifest::Manifest;
use crate::podman;

pub fn apply_command(file: String) -> Result<()> {
    let path = Path::new(&file);

    if !path.exists() {
        let message = format!("{} not exists", &file);
        return Err(anyhow!(message));
    }

    let content = fs::read_to_string(file)?;
    let parsed_manifest: Manifest = toml::from_str(&content)?;

    create_container(
        parsed_manifest.container_name.clone(),
        parsed_manifest.distro.clone(),
        parsed_manifest.default,
    )?;

    let commands = parsed_manifest.shell_hook.unwrap();

    for pkg in parsed_manifest.pkgs {
        install_command(pkg, Some(parsed_manifest.container_name.clone()))?;
    }

    for command in commands {
        podman::exec::exec_container(command, parsed_manifest.container_name.clone())?;
    }

    println!("Manifest executed!");

    Ok(())
}
