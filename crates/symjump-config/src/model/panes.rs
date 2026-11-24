// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

/// One preset grid. `cols` is left | right. `rows` is top / bottom.
/// `2x3` is 2 columns and 3 rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneLayout {
    pub label: String,
    pub cols: u32,
    pub rows: u32,
    pub keys: Option<String>,
}

impl PaneLayout {
    pub fn defaults() -> Vec<Self> {
        vec![
            Self {
                label: "2x2".into(),
                cols: 2,
                rows: 2,
                keys: None,
            },
            Self {
                label: "2x3".into(),
                cols: 2,
                rows: 3,
                keys: None,
            },
            Self {
                label: "3x2".into(),
                cols: 3,
                rows: 2,
                keys: None,
            },
        ]
    }

    pub fn check(&self) -> Result<(), String> {
        if self.label.trim().is_empty() || self.label.contains(['\t', '\n']) {
            return Err("pane layout needs a label".into());
        }
        if !(1..=6).contains(&self.cols) || !(1..=6).contains(&self.rows) {
            return Err(format!(
                "pane layout `{}`: cols and rows are 1 to 6",
                self.label
            ));
        }
        if self.cols * self.rows < 2 {
            return Err(format!(
                "pane layout `{}` needs at least 2 panes",
                self.label
            ));
        }
        if let Some(k) = &self.keys {
            let mut chars = k.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) if c.is_ascii_alphabetic() => {}
                _ => {
                    return Err(format!(
                        "pane layout `{}`: key must be one letter",
                        self.label
                    ));
                }
            }
        }
        Ok(())
    }
}
