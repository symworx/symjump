// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::collections::BTreeSet;

use symjump_config::Config;

use crate::error::CoreError;

/// GNU readline emacs-mode Meta letters we never steal for favorite jumps.
const READLINE_META_RESERVED: &[(char, &str)] = &[
    ('b', "readline M-b backward-word"),
    ('f', "readline M-f forward-word"),
    ('d', "readline M-d kill-word"),
    ('y', "readline M-y yank-pop"),
];

pub fn meta_letter(spec: &str) -> Option<char> {
    let rest = spec.strip_prefix("M-")?;
    let mut chars = rest.chars();
    let c = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    Some(c)
}

pub fn reserved_meta_letters(cfg: &Config) -> BTreeSet<char> {
    let mut out = BTreeSet::new();
    for spec in [&cfg.keys.leader, &cfg.keys.kids, &cfg.keys.actions] {
        if let Some(c) = meta_letter(spec) {
            out.insert(c);
        }
    }
    out.extend(READLINE_META_RESERVED.iter().map(|(c, _)| *c));
    out
}

fn reserved_reason(cfg: &Config, c: char) -> Option<String> {
    if meta_letter(&cfg.keys.leader) == Some(c) {
        return Some(format!("{} places", cfg.keys.leader));
    }
    if meta_letter(&cfg.keys.kids) == Some(c) {
        return Some(format!("{} kids", cfg.keys.kids));
    }
    if meta_letter(&cfg.keys.actions) == Some(c) {
        return Some(format!("{} actions", cfg.keys.actions));
    }
    READLINE_META_RESERVED
        .iter()
        .find(|(ch, _)| *ch == c)
        .map(|(_, why)| (*why).to_string())
}

/// Single-letter keys become `M-<letter>`. Reject sjmp chords, readline
/// motion/kill letters, and keys already used by another favorite.
pub fn check_favorite_key(
    cfg: &Config,
    key: &str,
    except_label: Option<&str>,
) -> Result<(), CoreError> {
    let k = key.trim();
    if k.is_empty() {
        return Ok(());
    }
    let mut chars = k.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if let Some(why) = reserved_reason(cfg, c) {
            return Err(CoreError::KeyReserved(k.into(), why));
        }
    }
    if let Some(other) = cfg.favorites.iter().find(|f| {
        f.keys.as_deref() == Some(k)
            && except_label.is_none_or(|lab| !f.label.eq_ignore_ascii_case(lab))
    }) {
        return Err(CoreError::KeyTaken(k.into(), other.label.clone()));
    }
    Ok(())
}

fn action_key_taken<'a>(cfg: &'a Config, key: &str, except_label: Option<&str>) -> Option<&'a str> {
    cfg.actions
        .agent
        .iter()
        .find(|a| {
            a.keys.as_deref() == Some(key)
                && except_label.is_none_or(|lab| !a.label.eq_ignore_ascii_case(lab))
        })
        .map(|a| a.label.as_str())
        .or_else(|| {
            cfg.actions
                .toolbox
                .iter()
                .find(|t| {
                    t.keys.as_deref() == Some(key)
                        && except_label.is_none_or(|lab| !t.label.eq_ignore_ascii_case(lab))
                })
                .map(|t| t.label.as_str())
        })
}

pub fn check_action_key(
    cfg: &Config,
    key: &str,
    except_label: Option<&str>,
) -> Result<(), CoreError> {
    let k = key.trim();
    if k.is_empty() {
        return Ok(());
    }
    if let Some(other) = action_key_taken(cfg, k, except_label) {
        return Err(CoreError::KeyTaken(k.into(), other.into()));
    }
    Ok(())
}
