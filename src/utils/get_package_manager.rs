use crate::models::{config::Config, package_manager_actions::PackageManagerAction};
use anyhow::Result;

pub fn get_package_manager(
    config: Config,
    distro: String,
    action: PackageManagerAction,
) -> Result<String> {
    let mut pm_command = String::new();

    for pm in config.package_manager {
        if pm.distros.contains(&distro) {
            pm_command += &pm.name;
            pm_command += " ";
            pm_command += match action {
                PackageManagerAction::Install => &pm.install_command,
                PackageManagerAction::Update => &pm.update_command,
                PackageManagerAction::Remove => &pm.remove_command,
            }
        }
    }

    Ok(pm_command)
}
