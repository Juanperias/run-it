use crate::{
    models::package_manager_actions::PackageManagerAction,
    podman,
    utils::{
        get_config::get_config, get_package_manager::get_package_manager, remove_ref::remove_ref,
    },
};
use anyhow::Result;

pub fn remove_command(package: String, container: Option<String>) -> Result<()> {
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
        container_or_default.image.clone(),
        PackageManagerAction::Remove,
    )?;
    let formatted_command = format!("{} {}", command, package);
    println!("Removing package...");
    podman::exec::exec_container(formatted_command, container_or_default.name.clone())?;

    println!("Removing reference....");
    remove_ref(package)?;

    Ok(())
}
