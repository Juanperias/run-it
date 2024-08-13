use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub containers: Vec<DistroContainer>,
    pub default_container: DistroContainer,
    pub package_manager: Vec<PackageManager>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DistroContainer {
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageManager {
    pub name: String,
    pub distros: Vec<String>,
    pub install_command: String,
    pub update_command: String,
    pub remove_command: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            containers: Vec::new(),
            default_container: DistroContainer {
                name: "".to_string(),
                image: "".to_string(),
            },
            package_manager: vec![
                PackageManager {
                    name: "apt".to_string(),
                    distros: vec!["debian".to_string(), "ubuntu".to_string()],
                    update_command: "update".to_string(),
                    install_command: "-y install".to_string(),
                    remove_command: "-y remove".to_string(),
                },
                PackageManager {
                    name: "pacman".to_string(),
                    distros: vec!["archlinux".to_string()],
                    update_command: "-Syu".to_string(),
                    install_command: "--noconfirm -S".to_string(),
                    remove_command: "--noconfirm -R".to_string(),
                },
            ],
        }
    }
}
