// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::path::{Path, PathBuf};

pub fn is_git_root(dir: &Path) -> bool {
    dir.join(".git").exists()
}

pub fn kids(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    const SKIP: &[&str] = &[
        "target",
        "node_modules",
        ".git",
        ".venv",
        "venv",
        "__pycache__",
    ];
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for ent in std::fs::read_dir(dir)? {
        let ent = ent?;
        let name = ent.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') || SKIP.contains(&name.as_ref()) {
            continue;
        }
        let p = ent.path();
        if p.is_dir() {
            out.push(p);
        }
    }
    out.sort();
    Ok(out)
}
