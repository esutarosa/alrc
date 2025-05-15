use anyhow::{Context, Result};
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;

pub fn add_aliases(
    suggestions: &[(String, String)],
    indices: Vec<usize>,
    shell: Option<String>,
) -> Result<()> {
    let base = BaseDirs::new().context("Failed to retrieve the home directory")?;
    let rc_file = match shell.as_deref() {
        Some("bash") => ".bashrc",
        _ => ".zshrc",
    };
    let path: PathBuf = base.home_dir().join(rc_file);

    let mut content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
    content.push_str("\n# alrs generated aliases\n");

    for idx in indices {
        if idx == 0 || idx > suggestions.len() {
            continue;
        }
        let (cmd, alias) = &suggestions[idx - 1];
        let line = format!("alias {}=\"{}\"\n", alias, cmd);
        content.push_str(&line);
        println!("✔ Added to {}: {}", rc_file, line.trim());
    }

    fs::write(&path, content).with_context(|| format!("Failed to write {:?}", path))?;
    Ok(())
}
