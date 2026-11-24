// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::Config;

/// `key  label  kind  payload`, with the first three columns padded like favorite lines.
/// The key is first so the letter column shares a left edge. Payload (cmd or toolbox
/// name) is last so it may contain spaces.
pub fn action_list_lines(cfg: &Config) -> Vec<String> {
    let mut rows: Vec<(String, String, String, String)> = Vec::new();
    for a in &cfg.actions.agent {
        rows.push((
            a.keys.as_deref().unwrap_or("-").to_string(),
            a.label.clone(),
            "agent".into(),
            a.cmd.clone(),
        ));
    }
    for t in &cfg.actions.toolbox {
        rows.push((
            t.keys.as_deref().unwrap_or("-").to_string(),
            t.label.clone(),
            "toolbox".into(),
            t.name.clone(),
        ));
    }
    let key_w = rows
        .iter()
        .map(|(k, _, _, _)| k.chars().count())
        .max()
        .unwrap_or(0);
    let label_w = rows
        .iter()
        .map(|(_, l, _, _)| l.chars().count())
        .max()
        .unwrap_or(0);
    let kind_w = rows
        .iter()
        .map(|(_, _, k, _)| k.chars().count())
        .max()
        .unwrap_or(0);
    rows.into_iter()
        .map(|(key, label, kind, payload)| {
            format!("{key:<key_w$}  {label:<label_w$}  {kind:<kind_w$}  {payload}")
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
        assert_eq!(lines[0], "g  grok-build  agent    grok");
        assert_eq!(lines[1], "c  codex       agent    codex -C {path}");
        assert_eq!(lines[2], "p  python      toolbox  dev-python");
        let col = lines[0].len() - "grok".len();
        assert_eq!(&lines[1][col..], "codex -C {path}");
        assert_eq!(&lines[2][col..], "dev-python");
    }
}
