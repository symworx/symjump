// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ConfigError;
use crate::model::Config;

impl Config {
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

#[cfg(test)]
mod tests {
    use super::*;

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
