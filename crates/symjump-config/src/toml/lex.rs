// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use crate::error::ConfigError;

#[derive(Clone, Copy)]
pub(super) enum Section {
    Root,
    Keys,
    Frequent,
    Ai,
    Favorite,
    Toolbox,
    Agent,
}

pub(super) enum Value {
    Str(String),
    Int(u32),
}

pub(super) fn strip_comment(line: &str) -> &str {
    let mut in_str = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_str = !in_str,
            '#' if !in_str => return &line[..i],
            _ => {}
        }
    }
    line
}

pub(super) fn parse_header(line: &str, line_no: usize) -> Result<Section, ConfigError> {
    let err = || ConfigError::Parse(format!("line {line_no}: bad header `{line}`"));
    if line.starts_with("[[") && line.ends_with("]]") {
        return match &line[2..line.len() - 2] {
            "favorites" => Ok(Section::Favorite),
            "actions.toolbox" => Ok(Section::Toolbox),
            "actions.agent" => Ok(Section::Agent),
            _ => Err(err()),
        };
    }
    if line.starts_with('[') && line.ends_with(']') {
        return match &line[1..line.len() - 1] {
            "keys" => Ok(Section::Keys),
            "frequent" => Ok(Section::Frequent),
            "ai" => Ok(Section::Ai),
            _ => Err(err()),
        };
    }
    Err(err())
}

pub(super) fn parse_kv(line: &str, line_no: usize) -> Result<(String, Value), ConfigError> {
    let eq = line
        .find('=')
        .ok_or_else(|| ConfigError::Parse(format!("line {line_no}: expected key = value")))?;
    let key = line[..eq].trim().to_string();
    let raw = line[eq + 1..].trim();
    let val = if raw.starts_with('"') {
        Value::Str(unquote(raw, line_no)?)
    } else {
        let n: u32 = raw
            .parse()
            .map_err(|_| ConfigError::Parse(format!("line {line_no}: bad value `{raw}`")))?;
        Value::Int(n)
    };
    Ok((key, val))
}

fn unquote(s: &str, line_no: usize) -> Result<String, ConfigError> {
    if !s.starts_with('"') || !s.ends_with('"') || s.len() < 2 {
        return Err(ConfigError::Parse(format!(
            "line {line_no}: expected quoted string"
        )));
    }
    Ok(s[1..s.len() - 1]
        .replace("\\\"", "\"")
        .replace("\\\\", "\\"))
}
