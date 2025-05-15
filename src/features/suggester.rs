use crate::features::{analyzer, writer};
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::collections::HashSet;

pub fn display_groups(groups: &[(String, usize)]) {
    println!("Found frequent commands:");
    for (i, (cmd, cnt)) in groups.iter().enumerate() {
        println!("[{}] {:<30} ({:>3} times)", i + 1, cmd, cnt);
    }
}

pub fn suggest(history: &[String]) -> Vec<(String, String)> {
    let groups = analyzer::analyze(history);
    let mut used = HashSet::new();
    let mut suggestions = Vec::new();

    for (cmd, _) in groups.into_iter().take(20) {
        let mut alias: String = cmd
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .collect();
        if used.contains(&alias) {
            for i in 1.. {
                let cand = format!("{}{}", alias, i);
                if !used.contains(&cand) {
                    alias = cand;
                    break;
                }
            }
        }
        used.insert(alias.clone());
        suggestions.push((cmd, alias));
    }

    suggestions
}

pub fn interactive_add(suggestions: &[(String, String)]) -> Result<()> {
    let items: Vec<String> = suggestions
        .iter()
        .map(|(cmd, alias)| format!("alias {}=\"{}\"", alias, cmd))
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select aliases to add")
        .items(&items)
        .interact()?;

    let indices = selections.into_iter().map(|i| i + 1).collect();
    writer::add_aliases(&suggestions, indices, None)?;
    Ok(())
}
