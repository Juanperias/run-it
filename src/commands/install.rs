use anyhow::Result;

use crate::{
    models::package_manager_actions::PackageManagerAction,
    podman,
    utils::{
        create_ref::create_ref, get_config::get_config, get_package_manager::get_package_manager,
    },
};

pub fn install_command(package: String, container: Option<String>) -> Result<()> {
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
    let command = get_package_manager(
        config,
        container_or_default.image,
        PackageManagerAction::Install,
    )?;
    println!("Installing package...");
    let formated_command = format!("{} {}", command, package);
    podman::exec::exec_container(formated_command, container_or_default.name.clone())?;

    println!("Creating ref");
    create_ref(package, container_or_default.name)?;

    Ok(())
}
