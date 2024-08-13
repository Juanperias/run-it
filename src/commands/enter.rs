use anyhow::Result;
use std::io;

use crate::{podman, utils::get_config::get_config};

pub fn enter_command(container: Option<String>) -> Result<()> {
    let config = get_config()?;
    let container_or_default = match container {
        Some(value) => config
            .containers
            .iter()
            .find(|c| c.name == value)
            .unwrap()
            .to_owned(),
        _ => config.clone().default_container,
    };

    loop {
        let mut command = String::new();
        let stdin = io::stdin();

        println!("Runit terminal");
        stdin.read_line(&mut command)?;

        podman::exec::exec_container(command, container_or_default.name.clone())?;
    }
}
