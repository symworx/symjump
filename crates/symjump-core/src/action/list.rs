// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::Config;

/// `kind  key  label  payload` with leading columns padded like favorite lines.
/// Payload (cmd or toolbox name) is last so it may contain spaces.
pub fn action_list_lines(cfg: &Config) -> Vec<String> {
    let mut rows: Vec<(String, String, String, String)> = Vec::new();
    for a in &cfg.actions.agent {
        rows.push((
            "agent".into(),
            a.keys.as_deref().unwrap_or("-").to_string(),
            a.label.clone(),
            a.cmd.clone(),
        ));
    }
    for t in &cfg.actions.toolbox {
        rows.push((
            "toolbox".into(),
            t.keys.as_deref().unwrap_or("-").to_string(),
            t.label.clone(),
            t.name.clone(),
        ));
    }
    let kind_w = rows
        .iter()
        .map(|(k, _, _, _)| k.chars().count())
        .max()
        .unwrap_or(0);
    let key_w = rows
        .iter()
        .map(|(_, k, _, _)| k.chars().count())
        .max()
        .unwrap_or(0);
    let label_w = rows
        .iter()
        .map(|(_, _, l, _)| l.chars().count())
        .max()
        .unwrap_or(0);
    rows.into_iter()
        .map(|(kind, key, label, payload)| {
            format!("{kind:<kind_w$}  {key:<key_w$}  {label:<label_w$}  {payload}")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::test_support::cfg;

    use super::action_list_lines;

    #[test]
    fn action_list_columns_align() {
        let lines = action_list_lines(&cfg());
        assert_eq!(lines[0], "agent    g  grok-build  grok");
        assert_eq!(lines[1], "agent    c  codex       codex -C {path}");
        assert_eq!(lines[2], "toolbox  p  python      dev-python");
        let col = lines[0].len() - "grok".len();
        assert_eq!(&lines[1][col..], "codex -C {path}");
        assert_eq!(&lines[2][col..], "dev-python");
    }
}
