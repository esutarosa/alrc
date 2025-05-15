use anyhow::{Context, Result};
use directories::BaseDirs;
use std::fs;

pub fn read_history(shell: Option<String>) -> Result<Vec<String>> {
    let base = BaseDirs::new().context("Failed to retrieve the home directory")?;
    let home = base.home_dir();
    let shells = if let Some(sh) = shell {
        vec![sh]
    } else {
        vec!["zsh".into(), "bash".into()]
    };

    let mut commands = Vec::new();
    for sh in shells {
        let path = match sh.as_str() {
            "zsh" => home.join(".zsh_history"),
            "bash" => home.join(".bash_history"),
            other => home.join(format!(".{}_history", other)),
        };
        if !path.exists() {
            continue;
        }
        let text =
            fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        for line in text.lines() {
            let raw = if line.starts_with(':') {
                if let Some(idx) = line.find(';') {
                    &line[idx + 1..]
                } else {
                    line
                }
            } else {
                line
            };
            let cmd = raw.trim();
            if !cmd.is_empty() {
                commands.push(cmd.to_string());
            }
        }
    }
    Ok(commands)
}
