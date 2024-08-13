use crate::utils::exec::exec;
use anyhow::Result;

pub fn create(name: String, image: String) -> Result<()> {
    let command = format!("podman run -d -it --name {} {}", name, image);

    exec(&command)?;
    Ok(())
}
