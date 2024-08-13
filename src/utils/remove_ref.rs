use anyhow::Result;
use std::fs;

pub fn remove_ref(package: String) -> Result<()> {
    let directory = dirs::home_dir().unwrap().join(".run-it");
    let file_dir = directory.join(&package);

    fs::remove_file(&file_dir)?;
    println!("The reference was deleted");

    Ok(())
}
