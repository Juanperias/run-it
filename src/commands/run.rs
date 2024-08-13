use crate::{podman, utils::get_config::get_config};
use anyhow::Result;

pub fn run_command(container: Option<String>, command: String) -> Result<()> {
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

    podman::start::start_container(container_or_default.name.clone())?;
    podman::exec::exec_container(command, container_or_default.name.clone())?;
    Ok(())
}
