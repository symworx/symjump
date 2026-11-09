// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

/// Split sizes for the agent dashboard.
///
/// `right` is the shell column (`tmux split-window -h -l`).
/// `bottom` is the lower shell's share of that column (`-v -l`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dashboard {
    pub right: String,
    pub bottom: String,
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            right: "34%".into(),
            bottom: "50%".into(),
        }
    }
}

impl Dashboard {
    /// A tmux `-l` size: `1`–`99%`, or `1`–`9999` cells. No leading zeros.
    pub fn parse_size(raw: &str) -> Result<String, String> {
        let s = raw.trim();
        let (digits, percent) = match s.strip_suffix('%') {
            Some(n) => (n, true),
            None => (s, false),
        };
        let bad = || format!("dashboard size must be 1-99% or 1-9999 cells (got `{raw}`)");
        if digits.is_empty()
            || digits.starts_with('0')
            || !digits.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(bad());
        }
        let n: u32 = match digits.parse() {
            Ok(n) => n,
            Err(_) => return Err(bad()),
        };
        let ok = if percent {
            (1..=99).contains(&n)
        } else {
            (1..=9999).contains(&n)
        };
        if !ok {
            return Err(bad());
        }
        Ok(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Dashboard;

    #[test]
    fn accepts_percent_and_cells() {
        for ok in ["1%", "34%", "99%", "1", "50", "9999", " 40% "] {
            assert!(Dashboard::parse_size(ok).is_ok(), "{ok}");
        }
        assert_eq!(Dashboard::parse_size(" 40% ").unwrap(), "40%");
    }

    #[test]
    fn rejects_sizes_tmux_must_not_see() {
        for bad in [
            "",
            "0",
            "0%",
            "00",
            "08",
            "100%",
            "10000",
            "34%;touch",
            "34 %",
            "-1",
        ] {
            assert!(Dashboard::parse_size(bad).is_err(), "{bad}");
        }
    }
}
