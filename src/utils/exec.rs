use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn exec(command: &str) -> Result<()> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("error running the command: {}", command))?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let formatted_line = format!("{}\n\r", line?);
            print!("{}", formatted_line);
        }
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Command failed: {}", command));
    }

    Ok(())
}
