// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::path::Path;

use symjump_config::Config;

pub fn ensure_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if Config::ensure_path(path)? {
        eprintln!("sjmp: created {}", path.display());
    }
    Ok(())
}

pub fn load_or_empty(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    if path.exists() {
        Ok(Config::load_path(path)?)
    } else {
        Ok(Config::default())
    }
}
