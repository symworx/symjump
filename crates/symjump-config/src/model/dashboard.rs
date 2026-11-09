// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

/// Which way the primary divider runs.
///
/// `vertical` is left | right. `horizontal` is top / bottom.
/// tmux names these the other way: left | right is `split-window -h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
    #[default]
    Vertical,
    Horizontal,
}

impl SplitDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }

    /// File value: `vertical` or `horizontal` only.
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim() {
            "vertical" => Ok(Self::Vertical),
            "horizontal" => Ok(Self::Horizontal),
            _ => Err(format!(
                "dashboard direction must be vertical or horizontal (got `{raw}`)"
            )),
        }
    }

    pub fn other(self) -> Self {
        match self {
            Self::Vertical => Self::Horizontal,
            Self::Horizontal => Self::Vertical,
        }
    }
}

/// One pane along the primary split. `subs` is empty, or the secondary percents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashPane {
    pub size: u8,
    pub subs: Vec<u8>,
}

/// Agent dashboard geometry.
///
/// The first pane keeps the agent. Further primary panes are shells.
/// A non-empty `subs` list splits that pane the other way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dashboard {
    pub direction: SplitDirection,
    pub panes: Vec<DashPane>,
    pub(crate) explicit_panes: bool,
    pub(crate) direction_set: bool,
    pub(crate) legacy_right: Option<u8>,
    pub(crate) legacy_bottom: Option<u8>,
}

impl Default for Dashboard {
    fn default() -> Self {
        // The previous layout was a 34% shell column, split in half.
        // That is 66/34 of the window, with 50/50 on the right only.
        Self {
            direction: SplitDirection::Vertical,
            panes: vec![
                DashPane {
                    size: 66,
                    subs: vec![],
                },
                DashPane {
                    size: 34,
                    subs: vec![50, 50],
                },
            ],
            explicit_panes: false,
            direction_set: false,
            legacy_right: None,
            legacy_bottom: None,
        }
    }
}

impl Dashboard {
    pub fn new(direction: SplitDirection, panes: Vec<DashPane>) -> Result<Self, String> {
        let dash = Self {
            direction,
            panes,
            explicit_panes: false,
            direction_set: false,
            legacy_right: None,
            legacy_bottom: None,
        };
        dash.check()?;
        Ok(dash)
    }

    pub fn set_direction(&mut self, raw: &str) -> Result<(), String> {
        self.direction = SplitDirection::parse(raw)?;
        self.direction_set = true;
        Ok(())
    }

    pub fn set_legacy_right(&mut self, raw: &str) -> Result<(), String> {
        self.legacy_right = Some(legacy_percent(raw)?);
        Ok(())
    }

    pub fn set_legacy_bottom(&mut self, raw: &str) -> Result<(), String> {
        self.legacy_bottom = Some(legacy_percent(raw)?);
        Ok(())
    }

    pub fn begin_pane(&mut self) {
        if !self.explicit_panes {
            self.panes.clear();
            self.explicit_panes = true;
        }
        self.panes.push(DashPane {
            size: 0,
            subs: vec![],
        });
    }

    pub fn set_pane_size(&mut self, raw: &str) -> Result<(), String> {
        let pane = self
            .panes
            .last_mut()
            .ok_or_else(|| "dashboard size needs a [[dashboard.pane]] table".to_string())?;
        pane.size = one_percent(raw, true)?;
        Ok(())
    }

    pub fn set_pane_sub(&mut self, raw: &str) -> Result<(), String> {
        let pane = self
            .panes
            .last_mut()
            .ok_or_else(|| "dashboard sub needs a [[dashboard.pane]] table".to_string())?;
        pane.subs = parse_full_percents(raw)?;
        Ok(())
    }

    /// Turn legacy `right` / `bottom` into panes, then check the geometry.
    pub fn normalize(&mut self) -> Result<(), String> {
        let legacy = self.legacy_right.is_some() || self.legacy_bottom.is_some();
        if legacy && self.explicit_panes {
            return Err("dashboard right/bottom cannot be combined with [[dashboard.pane]]".into());
        }
        if legacy && self.direction_set && self.direction != SplitDirection::Vertical {
            return Err(
                "dashboard right/bottom are a vertical split; remove them or set direction = \"vertical\""
                    .into(),
            );
        }
        if legacy {
            let right = self.legacy_right.unwrap_or(34);
            let bottom = self.legacy_bottom.unwrap_or(50);
            self.direction = SplitDirection::Vertical;
            self.panes = vec![
                DashPane {
                    size: 100 - right,
                    subs: vec![],
                },
                DashPane {
                    size: right,
                    subs: vec![100 - bottom, bottom],
                },
            ];
        }
        self.explicit_panes = false;
        self.direction_set = false;
        self.legacy_right = None;
        self.legacy_bottom = None;
        self.check()
    }

    pub fn check(&self) -> Result<(), String> {
        let n = self.panes.len();
        if !(1..=4).contains(&n) {
            return Err("dashboard needs 1-4 primary panes (0-3 splits)".into());
        }
        let mut sum = 0u32;
        for pane in &self.panes {
            if pane.size == 0 {
                return Err("dashboard pane is missing size".into());
            }
            if n == 1 {
                if pane.size != 100 {
                    return Err("a single dashboard pane must be 100".into());
                }
            } else if !(1..=99).contains(&pane.size) {
                return Err(format!(
                    "dashboard pane size must be 1-99 (got `{}`)",
                    pane.size
                ));
            }
            sum += u32::from(pane.size);
            if pane.subs.is_empty() {
                continue;
            }
            if pane.subs.len() < 2 || pane.subs.len() > 4 {
                return Err("dashboard secondary split count must be 1-3".into());
            }
            let mut sub = 0u32;
            for size in &pane.subs {
                if !(1..=99).contains(size) {
                    return Err(format!(
                        "dashboard secondary percent must be 1-99 (got `{size}`)"
                    ));
                }
                sub += u32::from(*size);
            }
            if sub != 100 {
                return Err(format!(
                    "dashboard secondary percents must sum to 100 (got {sub})"
                ));
            }
        }
        if sum != 100 {
            return Err(format!("dashboard percents must sum to 100 (got {sum})"));
        }
        Ok(())
    }

    /// Lines the hook reads. Integers only, so a bad value cannot reach tmux.
    pub fn layout(&self) -> String {
        let mut out = format!("{}\n", self.direction.as_str());
        for pane in &self.panes {
            out.push_str(&pane.size.to_string());
            if pane.subs.is_empty() {
                out.push_str(" 0\n");
            } else {
                out.push_str(&format!(" {}", pane.subs.len() - 1));
                for size in &pane.subs {
                    out.push_str(&format!(" {size}"));
                }
                out.push('\n');
            }
        }
        out
    }

    pub fn summary(&self) -> String {
        let ratio = ratio_text(&self.panes.iter().map(|p| p.size).collect::<Vec<_>>());
        let mut out = format!("{} {ratio}\n", self.direction.as_str());
        for (i, pane) in self.panes.iter().enumerate() {
            let name = pane_name(self.direction, i, self.panes.len());
            if pane.subs.is_empty() {
                out.push_str(&format!("  {name}\n"));
            } else {
                out.push_str(&format!("  {name} {}\n", ratio_text(&pane.subs)));
            }
        }
        out
    }
}

pub fn pane_name(direction: SplitDirection, index: usize, count: usize) -> &'static str {
    match (direction, count, index) {
        (_, 1, _) => "only",
        (SplitDirection::Vertical, 2, 0) => "left",
        (SplitDirection::Vertical, 2, _) => "right",
        (SplitDirection::Vertical, 3, 0) => "left",
        (SplitDirection::Vertical, 3, 1) => "middle",
        (SplitDirection::Vertical, 3, _) => "right",
        (SplitDirection::Vertical, 4, 0) => "left",
        (SplitDirection::Vertical, 4, 1) => "mid-left",
        (SplitDirection::Vertical, 4, 2) => "mid-right",
        (SplitDirection::Vertical, _, _) => "right",
        (SplitDirection::Horizontal, 2, 0) => "top",
        (SplitDirection::Horizontal, 2, _) => "bottom",
        (SplitDirection::Horizontal, 3, 0) => "top",
        (SplitDirection::Horizontal, 3, 1) => "middle",
        (SplitDirection::Horizontal, 3, _) => "bottom",
        (SplitDirection::Horizontal, 4, 0) => "top",
        (SplitDirection::Horizontal, 4, 1) => "upper",
        (SplitDirection::Horizontal, 4, 2) => "lower",
        (SplitDirection::Horizontal, _, _) => "bottom",
    }
}

pub fn ratio_text(parts: &[u8]) -> String {
    parts
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("/")
}

/// `60/40`, `60,40`, or `60 40`. A single number fills the last part when `n` is known.
pub fn parse_percents(raw: &str, n: usize) -> Result<Vec<u8>, String> {
    if !(1..=4).contains(&n) {
        return Err("dashboard needs 1-4 percents".into());
    }
    let mut nums = percent_tokens(raw)?;
    if nums.len() + 1 == n && n >= 2 {
        let sum: u32 = nums.iter().map(|v| u32::from(*v)).sum();
        if sum >= 100 {
            return Err(format!("dashboard percents must sum to 100 (got `{raw}`)"));
        }
        nums.push(u8::try_from(100 - sum).unwrap_or(0));
    }
    if nums.len() != n {
        return Err(format!(
            "dashboard needs {n} percents that sum to 100 (got `{raw}`)"
        ));
    }
    check_percents(&nums, raw)?;
    Ok(nums)
}

/// A complete ratio. The number of parts is the number of tokens.
pub fn parse_full_percents(raw: &str) -> Result<Vec<u8>, String> {
    let nums = percent_tokens(raw)?;
    if !(1..=4).contains(&nums.len()) {
        return Err(format!(
            "dashboard needs 1-4 percents that sum to 100 (got `{raw}`)"
        ));
    }
    check_percents(&nums, raw)?;
    Ok(nums)
}

fn percent_tokens(raw: &str) -> Result<Vec<u8>, String> {
    let bits: Vec<&str> = raw
        .trim()
        .split(|c: char| c == '/' || c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    if bits.is_empty() {
        return Err(format!("dashboard percents must sum to 100 (got `{raw}`)"));
    }
    let mut nums = Vec::with_capacity(bits.len());
    for bit in bits {
        nums.push(one_percent(bit, true)?);
    }
    Ok(nums)
}

fn one_percent(raw: &str, allow_100: bool) -> Result<u8, String> {
    let s = raw.trim().strip_suffix('%').unwrap_or(raw.trim());
    let bad = || format!("dashboard percent must be 1-99 (got `{raw}`)");
    if s.is_empty() || s.starts_with('0') || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(bad());
    }
    let n: u32 = s.parse().map_err(|_| bad())?;
    if n == 100 && allow_100 {
        return Ok(100);
    }
    if !(1..=99).contains(&n) {
        return Err(bad());
    }
    Ok(u8::try_from(n).unwrap_or(0))
}

fn check_percents(nums: &[u8], raw: &str) -> Result<(), String> {
    if nums.len() == 1 {
        if nums[0] != 100 {
            return Err("a single dashboard pane must be 100".into());
        }
        return Ok(());
    }
    for n in nums {
        if !(1..=99).contains(n) {
            return Err(format!("dashboard percent must be 1-99 (got `{raw}`)"));
        }
    }
    let sum: u32 = nums.iter().map(|n| u32::from(*n)).sum();
    if sum != 100 {
        return Err(format!("dashboard percents must sum to 100 (got `{raw}`)"));
    }
    Ok(())
}

fn legacy_percent(raw: &str) -> Result<u8, String> {
    // Older configs stored "30" or "30%". Both are a percent of the window.
    let s = raw.trim().strip_suffix('%').unwrap_or(raw.trim());
    one_percent(s, false)
        .map_err(|_| format!("dashboard right and bottom must be 1-99 (got `{raw}`)"))
}

#[cfg(test)]
mod tests {
    use super::{Dashboard, SplitDirection, pane_name, parse_full_percents, parse_percents};

    #[test]
    fn ratios_sum_to_100() {
        assert_eq!(parse_percents("60/40", 2).unwrap(), vec![60, 40]);
        assert_eq!(parse_percents("60", 2).unwrap(), vec![60, 40]);
        assert_eq!(parse_percents("50,30,20", 3).unwrap(), vec![50, 30, 20]);
        assert_eq!(parse_full_percents("50/50").unwrap(), vec![50, 50]);
        assert_eq!(parse_full_percents("100").unwrap(), vec![100]);
        assert!(parse_full_percents("60/50").is_err());
        assert!(parse_full_percents("60/40;touch").is_err());
        assert!(parse_percents("0/100", 2).is_err());
    }

    #[test]
    fn default_is_the_old_34_column() {
        let dash = Dashboard::default();
        assert_eq!(dash.direction, SplitDirection::Vertical);
        assert_eq!(dash.summary(), "vertical 66/34\n  left\n  right 50/50\n");
        assert_eq!(dash.layout(), "vertical\n66 0\n34 1 50 50\n");
        assert_eq!(pane_name(SplitDirection::Horizontal, 0, 2), "top");
    }
}
