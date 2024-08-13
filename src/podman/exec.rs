use anyhow::Result;

use crate::utils::exec::exec;

pub fn exec_container(command: String, container: String) -> Result<()> {
    let command = format!("podman exec -it {} {}", container, command);
    exec(&command)?;

    Ok(())
}
