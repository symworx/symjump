// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::env;

use symjump_config::expand_user;
use symjump_core::{is_git_root, kids};

use crate::ctx::Ctx;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let dir = match ctx.args.get(1) {
        Some(p) => expand_user(p, ctx.home),
        None => env::current_dir()?,
    };
    if is_git_root(&dir) {
        eprintln!(
            "sjmp: {} looks like a git project; listing kids anyway",
            dir.display()
        );
    }
    for p in kids(&dir)? {
        println!("{}", p.display());
    }
    Ok(())
}
