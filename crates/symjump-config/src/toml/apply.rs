// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use super::lex::{Section, Value};
use crate::error::ConfigError;
use crate::model::Config;

pub(super) fn apply_kv(
    cfg: &mut Config,
    section: Section,
    key: &str,
    val: Value,
    line_no: usize,
) -> Result<(), ConfigError> {
    let bad = || ConfigError::Parse(format!("line {line_no}: unexpected key `{key}`"));
    match section {
        Section::Root => match (key, val) {
            ("root", Value::Str(s)) => cfg.root = Some(s),
            _ => return Err(bad()),
        },
        Section::Keys => {
            let s = match val {
                Value::Str(s) => s,
                Value::Int(_) => return Err(bad()),
            };
            match key {
                "leader" => cfg.keys.leader = s,
                "kids" => cfg.keys.kids = s,
                "actions" => cfg.keys.actions = s,
                _ => return Err(bad()),
            }
        }
        Section::Frequent => match (key, val) {
            ("source", Value::Str(s)) => cfg.frequent.source = s,
            ("max", Value::Int(n)) => cfg.frequent.max = n,
            _ => return Err(bad()),
        },
        Section::Ai => {
            let s = match val {
                Value::Str(s) => s,
                Value::Int(_) => return Err(bad()),
            };
            match key {
                "default" => cfg.ai.default = s.trim().to_ascii_lowercase(),
                "local" => cfg.ai.local = s,
                _ => return Err(bad()),
            }
        }
        Section::Favorite => {
            let f = cfg.favorites.last_mut().ok_or_else(bad)?;
            match (key, val) {
                ("label", Value::Str(s)) => f.label = s,
                ("path", Value::Str(s)) => f.path = s,
                ("keys", Value::Str(s)) => f.keys = Some(s),
                _ => return Err(bad()),
            }
        }
        Section::Toolbox => {
            let t = cfg.actions.toolbox.last_mut().ok_or_else(bad)?;
            match (key, val) {
                ("label", Value::Str(s)) => t.label = s,
                ("name", Value::Str(s)) => t.name = s,
                ("keys", Value::Str(s)) => t.keys = Some(s),
                _ => return Err(bad()),
            }
        }
        Section::Agent => {
            let a = cfg.actions.agent.last_mut().ok_or_else(bad)?;
            match (key, val) {
                ("label", Value::Str(s)) => a.label = s,
                ("cmd", Value::Str(s)) => a.cmd = s,
                ("keys", Value::Str(s)) => a.keys = Some(s),
                _ => return Err(bad()),
            }
        }
    }
    Ok(())
}
