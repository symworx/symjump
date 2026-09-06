//! Config file types and I/O for symjump.
//!
//! Default path: `$XDG_CONFIG_HOME/symjump/favorites.toml`
//! (fallback `~/.config/symjump/favorites.toml`).

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml parse: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("toml serialize: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error("home directory not found")]
    NoHome,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default)]
    pub keys: Keys,
    #[serde(default)]
    pub frequent: Frequent,
    #[serde(default)]
    pub favorites: Vec<Favorite>,
    #[serde(default)]
    pub verbs: Verbs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Keys {
    #[serde(default = "default_leader")]
    pub leader: String,
    #[serde(default = "default_kids")]
    pub kids: String,
    #[serde(default = "default_verbs")]
    pub verbs: String,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            leader: default_leader(),
            kids: default_kids(),
            verbs: default_verbs(),
        }
    }
}

fn default_leader() -> String {
    "M-p".into()
}
fn default_kids() -> String {
    "M-P".into()
}
fn default_verbs() -> String {
    "M-x".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frequent {
    #[serde(default = "default_freq_source")]
    pub source: String,
    #[serde(default = "default_freq_max")]
    pub max: u32,
}

impl Default for Frequent {
    fn default() -> Self {
        Self {
            source: default_freq_source(),
            max: default_freq_max(),
        }
    }
}

fn default_freq_source() -> String {
    "zoxide".into()
}
fn default_freq_max() -> u32 {
    20
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Favorite {
    pub label: String,
    pub path: String,
    #[serde(default)]
    pub keys: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Verbs {
    #[serde(default)]
    pub toolbox: Vec<ToolboxVerb>,
    #[serde(default)]
    pub agent: Vec<AgentVerb>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolboxVerb {
    pub label: String,
    pub name: String,
    #[serde(default)]
    pub keys: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentVerb {
    pub label: String,
    pub cmd: String,
    #[serde(default)]
    pub keys: Option<String>,
}

impl Config {
    pub fn parse_str(s: &str) -> Result<Self, ConfigError> {
        Ok(toml::from_str(s)?)
    }

    pub fn to_toml(&self) -> Result<String, ConfigError> {
        Ok(toml::to_string_pretty(self)?)
    }

    pub fn default_path() -> Result<PathBuf, ConfigError> {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                return Ok(PathBuf::from(xdg).join("symjump").join("favorites.toml"));
            }
        }
        let home = std::env::var("HOME").map_err(|_| ConfigError::NoHome)?;
        Ok(PathBuf::from(home).join(".config").join("symjump").join("favorites.toml"))
    }

    pub fn load_path(path: &Path) -> Result<Self, ConfigError> {
        let raw = fs::read_to_string(path)?;
        Self::parse_str(&raw)
    }

    pub fn save_path(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, self.to_toml()?)?;
        Ok(())
    }
}

/// Expand a leading `~` or `$HOME` using the given home directory.
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
root = "~/worx"

[keys]
leader = "M-p"
kids = "M-P"
verbs = "M-x"

[frequent]
source = "zoxide"
max = 20

[[favorites]]
label = "worx"
path = "~/worx"
keys = "w"

[[favorites]]
label = "symworx"
path = "~/worx/symworx"
keys = "s"

[[verbs.toolbox]]
label = "python"
name = "dev-python"
keys = "p"

[[verbs.agent]]
label = "grok-build"
keys = "g"
cmd = "grok"
"#;

    #[test]
    fn parse_design_sample() {
        let c = Config::parse_str(SAMPLE).unwrap();
        assert_eq!(c.root.as_deref(), Some("~/worx"));
        assert_eq!(c.keys.leader, "M-p");
        assert_eq!(c.keys.verbs, "M-x");
        assert_eq!(c.favorites.len(), 2);
        assert_eq!(c.favorites[1].label, "symworx");
        assert_eq!(c.favorites[1].keys.as_deref(), Some("s"));
        assert_eq!(c.verbs.toolbox[0].name, "dev-python");
        assert_eq!(c.verbs.agent[0].cmd, "grok");
    }

    #[test]
    fn empty_toml_gets_defaults() {
        let c = Config::parse_str("").unwrap();
        assert_eq!(c.keys.leader, "M-p");
        assert_eq!(c.keys.verbs, "M-x");
        assert!(c.favorites.is_empty());
        assert_eq!(c.frequent.source, "zoxide");
    }

    #[test]
    fn roundtrip_toml() {
        let c = Config::parse_str(SAMPLE).unwrap();
        let again = Config::parse_str(&c.to_toml().unwrap()).unwrap();
        assert_eq!(c, again);
    }

    #[test]
    fn expand_tilde_and_home() {
        let home = Path::new("/home/nate");
        assert_eq!(expand_user("~/worx", home), PathBuf::from("/home/nate/worx"));
        assert_eq!(expand_user("~", home), PathBuf::from("/home/nate"));
        assert_eq!(
            expand_user("$HOME/worx/symworx", home),
            PathBuf::from("/home/nate/worx/symworx")
        );
        assert_eq!(expand_user("/abs/path", home), PathBuf::from("/abs/path"));
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!("symjump-cfg-{}", std::process::id()));
        let path = dir.join("favorites.toml");
        let mut c = Config::default();
        c.favorites.push(Favorite {
            label: "x".into(),
            path: "~/x".into(),
            keys: Some("x".into()),
        });
        c.save_path(&path).unwrap();
        let loaded = Config::load_path(&path).unwrap();
        assert_eq!(loaded.favorites[0].label, "x");
        let _ = fs::remove_dir_all(&dir);
    }
}
