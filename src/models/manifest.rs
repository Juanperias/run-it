use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub pkgs: Vec<String>,
    pub container_name: String,
    pub distro: String,
    pub default: bool,
}
