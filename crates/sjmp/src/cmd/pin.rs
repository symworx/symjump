// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::env;
use std::path::PathBuf;

use symjump_core::pin;

use crate::ctx::Ctx;
use crate::help::{self, print_pin};
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    if help::wants_help(ctx.args, 1) {
        print_pin();
        return Ok(());
    }
    let mut cfg = load::load_or_empty(ctx.cfg_path)?;
    let mut path: Option<PathBuf> = None;
    let mut label: Option<String> = None;
    let mut keys: Option<String> = None;
    let mut current = false;
    let mut i = 1;
    while i < ctx.args.len() {
        match ctx.args[i].as_str() {
            "--label" => {
                i += 1;
                label = ctx.args.get(i).cloned();
            }
            "--keys" => {
                i += 1;
                keys = ctx.args.get(i).cloned();
            }
            "--current" => current = true,
            s if !s.starts_with('-') => path = Some(PathBuf::from(s)),
            other => return Err(format!("unknown pin flag: {other}").into()),
        }
        i += 1;
    }
    if current && path.is_some() {
        return Err("pin: --current cannot be combined with a path".into());
    }
    let raw = match path {
        Some(p) => p,
        None => env::current_dir()?,
    };
    let abs = if raw.is_absolute() {
        raw
    } else {
        env::current_dir()?.join(raw)
    };
    let label = label.unwrap_or_else(|| {
        abs.file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "pin".into())
    });
    pin(&mut cfg, label, abs.to_string_lossy().into_owned(), keys)?;
    cfg.save_path(ctx.cfg_path)?;
    println!("{}", abs.display());
    Ok(())
}
