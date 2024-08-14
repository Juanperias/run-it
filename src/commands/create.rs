use crate::{
    models::{config::DistroContainer, package_manager_actions::PackageManagerAction},
    podman,
    utils::{
        get_config::get_config, get_package_manager::get_package_manager,
        write_config::write_config,
    },
};
use anyhow::Result;

pub fn create_container(name: String, distro: String, use_default: bool) -> Result<()> {
    let mut config = get_config()?;
    let container = DistroContainer {
        name,
        image: distro.clone(),
    };

    println!("Creating your container... this can take some minutes");
    let image = format!("docker.io/{}", &distro);

    if let Ok(_) = podman::container::create(container.name.clone(), image) {
        println!("Updating the container");
        podman::start::start_container(container.name.clone())?;
        let command =
            get_package_manager(config.clone(), distro.clone(), PackageManagerAction::Update)?;
        podman::exec::exec_container(command, container.name.clone())?;

        println!("Your container is created");

        config.containers.push(container.clone());

        if use_default {
            config.default_container = container.clone();
        }

        write_config(config)?;
    } else {
        println!("this container has already been created!");
    }

    Ok(())
}
