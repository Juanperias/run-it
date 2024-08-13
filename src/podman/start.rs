use crate::utils::exec::exec;
use anyhow::Result;

pub fn start_container(container: String) -> Result<()> {
    let message = format!("podman start {}", container);
    exec(&message)?;

    Ok(())
}
