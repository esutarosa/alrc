use std::collections::HashMap;

pub fn analyze(history: &[String]) -> Vec<(String, usize)> {
    let mut counts: HashMap<&String, usize> = HashMap::new();
    for cmd in history {
        *counts.entry(cmd).or_default() += 1;
    }

    let mut groups: Vec<(String, usize)> = counts
        .into_iter()
        .filter_map(|(cmd, count)| {
            if cmd.len() > 20 && count > 1 {
                Some((cmd.clone(), count))
            } else {
                None
            }
        })
        .collect();

    groups.sort_by(|a, b| b.1.cmp(&a.1));
    groups
}
