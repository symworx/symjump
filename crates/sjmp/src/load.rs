// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::path::Path;

use symjump_config::Config;

/// Create the config when missing. Returns true when this call wrote the file.
pub fn ensure_config(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    if Config::ensure_path(path)? {
        eprintln!("sjmp: created {}", path.display());
        return Ok(true);
    }
    Ok(false)
}

pub fn load_or_empty(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    if path.exists() {
        Ok(Config::load_path(path)?)
    } else {
        Ok(Config::default())
    }
}
