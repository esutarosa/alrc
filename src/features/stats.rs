use anyhow::Result;
use std::collections::HashMap;

pub fn show(history: &[String]) -> Result<()> {
    let mut counts: HashMap<&String, usize> = HashMap::new();
    for cmd in history {
        *counts.entry(cmd).or_default() += 1;
    }

    let mut freq: Vec<(&String, usize)> = counts.into_iter().collect();
    freq.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top {} commands by usage:", freq.len().min(10));
    for (cmd, count) in freq.into_iter().take(10) {
        println!("{:>5} : {}", count, cmd);
    }

    Ok(())
}
