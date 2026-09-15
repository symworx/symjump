// Copyright (c) 2026 PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

//! Config file types and I/O for symjump.
//!
//! Parser is a closed subset of TOML (no serde): root keys, `[keys]`,
//! `[frequent]`, `[[favorites]]`, `[[actions.toolbox]]`, `[[actions.agent]]`.

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("config parse: {0}")]
    Parse(String),
    #[error("home directory not found")]
    NoHome,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Config {
    pub root: Option<String>,
    pub keys: Keys,
    pub frequent: Frequent,
    pub favorites: Vec<Favorite>,
    pub actions: Actions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keys {
    pub leader: String,
    pub kids: String,
    pub actions: String,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            leader: "M-p".into(),
            kids: "M-P".into(),
            actions: "M-x".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frequent {
    pub source: String,
    pub max: u32,
}

impl Default for Frequent {
    fn default() -> Self {
        Self {
            source: "zoxide".into(),
            max: 20,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Favorite {
    pub label: String,
    pub path: String,
    pub keys: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Actions {
    pub toolbox: Vec<ToolboxAction>,
    pub agent: Vec<AgentAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolboxAction {
    pub label: String,
    pub name: String,
    pub keys: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentAction {
    pub label: String,
    pub cmd: String,
    pub keys: Option<String>,
}

#[derive(Clone, Copy)]
enum Section {
    Root,
    Keys,
    Frequent,
    Favorite,
    Toolbox,
    Agent,
}

impl Config {
    pub fn parse_str(s: &str) -> Result<Self, ConfigError> {
        parse_config(s)
    }

    pub fn to_toml(&self) -> Result<String, ConfigError> {
        Ok(emit_config(self))
    }

    pub fn default_path() -> Result<PathBuf, ConfigError> {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                return Ok(PathBuf::from(xdg).join("symjump").join("favorites.toml"));
            }
        }
        let home = std::env::var("HOME").map_err(|_| ConfigError::NoHome)?;
        Ok(PathBuf::from(home)
            .join(".config")
            .join("symjump")
            .join("favorites.toml"))
    }

    pub fn load_path(path: &Path) -> Result<Self, ConfigError> {
        Self::parse_str(&fs::read_to_string(path)?)
    }

    pub fn save_path(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, self.to_toml()?)?;
        Ok(())
    }

    /// Write a default config if `path` does not exist. Returns true when created.
    pub fn ensure_path(path: &Path) -> Result<bool, ConfigError> {
        if path.exists() {
            return Ok(false);
        }
        Config::default().save_path(path)?;
        Ok(true)
    }
}

pub fn expand_user(path: &str, home: &Path) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        return home.join(rest);
    }
    if path == "~" {
        return home.to_path_buf();
    }
    if let Some(rest) = path.strip_prefix("$HOME/") {
        return home.join(rest);
    }
    if path == "$HOME" {
        return home.to_path_buf();
    }
    PathBuf::from(path)
}

fn parse_config(s: &str) -> Result<Config, ConfigError> {
    let mut cfg = Config::default();
    let mut section = Section::Root;
    for (i, raw) in s.lines().enumerate() {
        let line_no = i + 1;
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            section = parse_header(line, line_no)?;
            match section {
                Section::Favorite => cfg.favorites.push(Favorite {
                    label: String::new(),
                    path: String::new(),
                    keys: None,
                }),
                Section::Toolbox => cfg.actions.toolbox.push(ToolboxAction {
                    label: String::new(),
                    name: String::new(),
                    keys: None,
                }),
                Section::Agent => cfg.actions.agent.push(AgentAction {
                    label: String::new(),
                    cmd: String::new(),
                    keys: None,
                }),
                _ => {}
            }
            continue;
        }
        let (key, val) = parse_kv(line, line_no)?;
        apply_kv(&mut cfg, section, &key, val, line_no)?;
    }
    Ok(cfg)
}

fn strip_comment(line: &str) -> &str {
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

fn parse_header(line: &str, line_no: usize) -> Result<Section, ConfigError> {
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
            _ => Err(err()),
        };
    }
    Err(err())
}

fn parse_kv(line: &str, line_no: usize) -> Result<(String, Value), ConfigError> {
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

enum Value {
    Str(String),
    Int(u32),
}

fn apply_kv(
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

fn emit_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn emit_config(cfg: &Config) -> String {
    let mut out = String::new();
    if let Some(root) = &cfg.root {
        out.push_str(&format!("root = {}\n\n", emit_string(root)));
    }
    out.push_str("[keys]\n");
    out.push_str(&format!("leader = {}\n", emit_string(&cfg.keys.leader)));
    out.push_str(&format!("kids = {}\n", emit_string(&cfg.keys.kids)));
    out.push_str(&format!("actions = {}\n\n", emit_string(&cfg.keys.actions)));
    out.push_str("[frequent]\n");
    out.push_str(&format!("source = {}\n", emit_string(&cfg.frequent.source)));
    out.push_str(&format!("max = {}\n\n", cfg.frequent.max));
    for f in &cfg.favorites {
        out.push_str("[[favorites]]\n");
        out.push_str(&format!("label = {}\n", emit_string(&f.label)));
        out.push_str(&format!("path = {}\n", emit_string(&f.path)));
        if let Some(k) = &f.keys {
            out.push_str(&format!("keys = {}\n", emit_string(k)));
        }
        out.push('\n');
    }
    for t in &cfg.actions.toolbox {
        out.push_str("[[actions.toolbox]]\n");
        out.push_str(&format!("label = {}\n", emit_string(&t.label)));
        out.push_str(&format!("name = {}\n", emit_string(&t.name)));
        if let Some(k) = &t.keys {
            out.push_str(&format!("keys = {}\n", emit_string(k)));
        }
        out.push('\n');
    }
    for a in &cfg.actions.agent {
        out.push_str("[[actions.agent]]\n");
        out.push_str(&format!("label = {}\n", emit_string(&a.label)));
        out.push_str(&format!("cmd = {}\n", emit_string(&a.cmd)));
        if let Some(k) = &a.keys {
            out.push_str(&format!("keys = {}\n", emit_string(k)));
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
root = "~/src"

[keys]
leader = "M-p"
kids = "M-P"
actions = "M-x"

[frequent]
source = "zoxide"
max = 20

[[favorites]]
label = "src"
path = "~/src"
keys = "r"

[[favorites]]
label = "symworx"
path = "~/src/symworx"
keys = "s"

[[actions.toolbox]]
label = "python"
name = "dev-python"
keys = "p"

[[actions.agent]]
label = "grok-build"
keys = "g"
cmd = "grok"
"#;

    #[test]
    fn parse_design_sample() {
        let c = Config::parse_str(SAMPLE).unwrap();
        assert_eq!(c.root.as_deref(), Some("~/src"));
        assert_eq!(c.keys.leader, "M-p");
        assert_eq!(c.favorites.len(), 2);
        assert_eq!(c.actions.agent[0].cmd, "grok");
    }

    #[test]
    fn empty_toml_gets_defaults() {
        let c = Config::parse_str("").unwrap();
        assert_eq!(c.keys.leader, "M-p");
        assert!(c.favorites.is_empty());
    }

    #[test]
    fn comments_and_unknown_header_fail() {
        assert_eq!(
            Config::parse_str("root = \"~/x\" # c\n")
                .unwrap()
                .root
                .as_deref(),
            Some("~/x")
        );
        assert!(Config::parse_str("[nope]\n").is_err());
        assert!(Config::parse_str("[[verbs.agent]]\n").is_err());
    }

    #[test]
    fn roundtrip_toml() {
        let c = Config::parse_str(SAMPLE).unwrap();
        assert_eq!(c, Config::parse_str(&c.to_toml().unwrap()).unwrap());
    }

    #[test]
    fn expand_tilde_and_home() {
        let home = Path::new("/home/user");
        assert_eq!(expand_user("~/src", home), PathBuf::from("/home/user/src"));
        assert_eq!(expand_user("$HOME/a", home), PathBuf::from("/home/user/a"));
    }

    #[test]
    fn ensure_path_writes_once() {
        let dir = std::env::temp_dir().join(format!("symjump-ensure-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let path = dir.join("symjump").join("favorites.toml");
        assert!(Config::ensure_path(&path).unwrap());
        let first = fs::read_to_string(&path).unwrap();
        assert!(first.contains("[keys]"));
        assert!(first.contains("actions"));
        fs::write(&path, "root = \"~/kept\"\n").unwrap();
        assert!(!Config::ensure_path(&path).unwrap());
        assert_eq!(fs::read_to_string(&path).unwrap(), "root = \"~/kept\"\n");
        let _ = fs::remove_dir_all(&dir);
    }
}
